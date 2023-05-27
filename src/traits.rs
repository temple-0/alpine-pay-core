#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps,
    DepsMut,
    Env,
    MessageInfo,
    Response,  
    StdResult,
    Addr
};

use crate::msg::{
    MultiDonationResponse, 
    SingleDonationResponse,
    NumDonationsResponse,
    UsernameAvailableResponse,
    MultiUserResponse,
    AlpineUserResponse
};
use crate::state::AlpineUser;

use crate::ContractError;

pub trait Donation: DonationQuery + DonationExecute { }

pub trait DonationQuery {
    fn get_sent_donations(&self, deps: Deps, sender: String) -> StdResult<MultiDonationResponse>;
    fn get_received_donations(&self, deps: Deps, recipient: String) -> StdResult<MultiDonationResponse>;
    fn get_single_donation(&self, deps: Deps, id: u64) -> StdResult<SingleDonationResponse>;
    fn get_num_donations(&self, deps: Deps) -> StdResult<NumDonationsResponse>;
    fn is_username_available(&self, deps: Deps, username: String) -> StdResult<UsernameAvailableResponse>;
    fn get_all_users(&self, deps: Deps) -> StdResult<MultiUserResponse>;
    fn get_user_by_addr(&self, deps: Deps, address: Addr) -> StdResult<AlpineUserResponse>;
    fn get_user_by_name(&self, deps: Deps, username: String) -> StdResult<AlpineUserResponse>;
}

pub trait DonationExecute{
    fn send_donation(
        &self,
        deps: DepsMut, 
        _env: Env, 
        info: MessageInfo, 
        sender: String,
        recipient: String, 
        message: String
    ) -> Result<Response, ContractError>;
    fn register_user(
        &self,
        deps: DepsMut,
        _env: Env,
        user: AlpineUser,
        username: String
    ) -> Result<Response, ContractError>;
}
