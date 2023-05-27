#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Binary, 
    Deps, 
    Env, 
    StdResult, 
    Order, 
    to_binary,
    Addr,
    Timestamp
};

use crate::msg::{
    QueryMsg, 
    MultiDonationResponse, 
    SingleDonationResponse,
    NumDonationsResponse,
    UsernameAvailableResponse,
    MultiUserResponse,
    AlpineUserResponse
};
use crate::state::{ AlpineContract, AlpineUser, DonationInfo };
use crate::traits::DonationQuery;

impl<'a> DonationQuery for AlpineContract<'a>
{
    fn get_num_donations(&self, deps: Deps) -> StdResult<NumDonationsResponse> {
        let count = self.donation_count(deps.storage)?;
        Ok(NumDonationsResponse { count })
    }

    fn get_sent_donations(&self, deps: Deps, sender: String) -> StdResult<MultiDonationResponse> {
        let sender_user = self.find_alpine_username(deps.storage, sender).unwrap();

        //Generate a vector of tuples containing the donation and a byte array identifier.
        let donations: StdResult<Vec<(Vec<_>, _)>> = self
            .donations
            .idx
            .sender
            .prefix(sender_user)
            .range(deps.storage, None, None, Order::Ascending)
            .collect();
        let donations = sort_donations_by_date(donations?.clone());

        Ok(MultiDonationResponse{ donations })
    }

    fn get_received_donations(&self, deps: Deps, recipient: String) -> StdResult<MultiDonationResponse> {
        let recipient_user = self.find_alpine_username(deps.storage, recipient).unwrap();

        // Generate a vector of tuples containing the donation and a byte array identifier
        let donations: StdResult<Vec<(Vec<_>, _)>> = self
            .donations
            .idx
            .recipient
            .prefix(recipient_user)
            .range(deps.storage, None, None, Order::Ascending)
            .collect();
        let donations = sort_donations_by_date(donations?.clone());

        Ok(MultiDonationResponse{ donations })
    }

    fn get_single_donation(&self, deps: Deps, id: u64) -> StdResult<SingleDonationResponse> {
        let donation = self.donations.load(deps.storage, &id.to_string())?;
        Ok(SingleDonationResponse{ donation })
    }

    fn is_username_available(&self, deps: Deps, username: String) -> StdResult<UsernameAvailableResponse> {
        let is_available = !self.contains_username(deps.storage, username);
        Ok(UsernameAvailableResponse { is_available })
    }
    
    fn get_all_users(&self, deps: Deps) -> StdResult<MultiUserResponse> {
        let usernames: StdResult<Vec<(String, _)>> = self
            .usernames
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .collect();
        let usernames = usernames?;

        let mut users: Vec<AlpineUser> = Vec::new();
        for username in usernames{
            users.push(username.1);
        }

        Ok(MultiUserResponse{ users })
    }

    fn get_user_by_addr(&self, deps: Deps, address: Addr) -> StdResult<AlpineUserResponse>{
        let user = match self.addresses.may_load(deps.storage, address.clone())? {
            Some(user) => { user },
            None => { AlpineUser::new(deps, address, None).unwrap() }
        };
        Ok(AlpineUserResponse{ user })
    }
    
    fn get_user_by_name(&self, deps: Deps, username: String) -> StdResult<AlpineUserResponse> {
        let user = match self.find_alpine_username(deps.storage, username.clone()) {
            Ok(user) => { user },
            Err(_) => { AlpineUser::empty() }
        };

        Ok(AlpineUserResponse { user })
    }
}

// route queries to the smart contract
impl<'a> AlpineContract<'a> {
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetSentDonations{ sender } => to_binary(&self.get_sent_donations(deps, sender)?),
            QueryMsg::GetReceivedDonations { recipient } => to_binary(&self.get_received_donations(deps, recipient)?),
            QueryMsg::GetSingleDonation { id } => to_binary(&self.get_single_donation(deps, id)?),
            QueryMsg::IsUsernameAvailable { username } => to_binary(&self.is_username_available(deps, username)?),
            QueryMsg::GetAllUsers { } => to_binary(&self.get_all_users(deps)?),
            QueryMsg::GetUserByAddr { address } => to_binary(&self.get_user_by_addr(deps, address)?),
            QueryMsg::GetUserByName { username } => to_binary(&self.get_user_by_name(deps, username)?)
        }
    }
}

fn sort_donations_by_date(mut donations: Vec<(Vec<u8>, DonationInfo)>) -> Vec<(Vec<u8>, DonationInfo)>{
    donations.sort_by(|a, b| {
        let a_timestamp = match a.1.timestamp{
            Some(time) => time,
            None => Timestamp::from_seconds(0)
        };
        let b_timestamp = match b.1.timestamp{
            Some(time) => time,
            None => Timestamp::from_seconds(0)
        };

        a_timestamp.cmp(&b_timestamp)
    });
    return donations;
}