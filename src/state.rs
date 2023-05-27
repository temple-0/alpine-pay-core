use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::iter::Iterator;

use cosmwasm_std::{
    Deps,
    Addr, 
    Storage, 
    StdResult,
    Timestamp,
    Order
};
use cw_storage_plus::{
  Item, 
  MultiIndex,
  IndexList, 
  Index, 
  IndexedMap,
  Map,
  PrimaryKey,
  Key,
  KeyDeserialize,
  Prefixer
};

use crate::traits::Donation;
use crate::error::ContractError;

pub struct AlpineContract<'a> {
    pub donation_count: Item<'a, u64>,
    pub donations: IndexedMap<'a, &'a str, DonationInfo, DonationIndexes<'a>>,
    pub usernames: Map<'a, String, AlpineUser>,
    pub addresses: Map<'a, Addr, AlpineUser>
}

impl<'a> Donation for AlpineContract<'a> { }

impl Default for AlpineContract<'static> {
    fn default() -> Self {
        Self::new(
            "num_donations",
            "donations",
            "usernames",
            "addresses"
        )
    }
}

impl<'a> AlpineContract<'a> {
    fn new(
        donation_count_key: &'a str,
        donations: &'a str,
        usernames: &'a str,
        addresses: &'a str
    ) -> Self {
        let indexes = DonationIndexes {
            sender: MultiIndex::new(|d| d.sender.clone(), donations, "donations__sender"),
            recipient: MultiIndex::new(|d| d.recipient.clone(), donations, "donations__recipient"),
        };
        Self {
            donation_count: Item::new(donation_count_key),
            donations: IndexedMap::new(donations, indexes),
            usernames: Map::new(usernames),
            addresses: Map::new(addresses)
        }
    }

    pub fn donation_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.donation_count.may_load(storage)?.unwrap_or_default())
    }

    pub fn increment_donations(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.donation_count(storage)? + 1;
        self.donation_count.save(storage, &val)?;
        Ok(val)
    }

    pub fn find_alpine_username(&self, storage: &dyn Storage, username: String) -> Result<AlpineUser, ContractError> {
        let mut usernames = self.usernames.keys(
            storage,
            None,
            None,
            Order::Descending
        );

        let found = match (*usernames).into_iter().filter(
            |u| u.as_deref().unwrap().to_lowercase() == username.to_lowercase()).next() {
                Some(user) => user?,
                None => String::from("")
            };

        let alpine_user = match self.usernames.may_load(storage, found.clone())? {
            Some(user) => user,
            None => return Err(ContractError::UserNotFound { user: username })
        };

        Ok(alpine_user)
    }

    pub fn contains_username(&self, storage: &dyn Storage, username: String) -> bool {
        let usernames: Vec<Result<std::string::String, cosmwasm_std::StdError>> = self.usernames.keys(
            storage,
            None,
            None,
            Order::Descending
        ).into_iter().collect();

        let search_result = usernames.iter().any(|u| u.as_ref().unwrap().to_lowercase() == username.to_lowercase());

        search_result
    }

    pub fn get_user_by_address(&self, storage: &dyn Storage, address: Addr) -> Result<AlpineUser, ContractError> {
        let alpine_user = match self.addresses.may_load(storage, address.clone())? {
            Some(user) => user,
            None => return Err(ContractError::UserNotFound { user: address.to_string() })
        };

        Ok(alpine_user)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AlpineUser {
    pub username: String,
    pub address: Addr
}

impl AlpineUser {
    pub fn new(deps: Deps, address: Addr, username: Option<String>) -> Result<AlpineUser, ContractError> {
        let address = match deps.api.addr_validate(address.as_str()) {
            Ok(addr) => addr,
            Err(_) => return Err(ContractError::InvalidWalletAddress { address: address.to_string() })
        };
        
        let username = match username {
            Some(name) => name,
            None => String::from("")
        };
        
        Ok(AlpineUser { username, address })
    }

    pub fn empty() -> AlpineUser {
        AlpineUser { username: String::from(""), address: Addr::unchecked("") }
    }
}

impl KeyDeserialize for &AlpineUser {
    type Output = Addr;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(Addr::unchecked(String::from_vec(value)?))
    }
}

impl<'a> PrimaryKey<'a> for AlpineUser {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = &'a AlpineUser;
    type SuperSuffix = &'a AlpineUser;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.address.as_bytes())]
    }
}

impl<'a> Prefixer<'a> for AlpineUser {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.address.as_bytes())]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationInfo {
    pub sender: AlpineUser,
    pub recipient: AlpineUser,
    pub amount: Vec<cosmwasm_std::Coin>,
    pub message: String,
    pub timestamp: Option<Timestamp>
}

// Creates a couple of indexes that we can use to search our indexed map
pub struct DonationIndexes<'a>{
    // Allows search results with multiple values for sender/receiver (S/R).
    // A S/R has an index of the S/R's address and the ID of the donation as bytes
    pub sender: MultiIndex<'a, AlpineUser, DonationInfo, Vec<u8>>,
    pub recipient: MultiIndex<'a, AlpineUser, DonationInfo, Vec<u8>>,
}

// Boilerplate code which builds a list of indexes
impl<'a> IndexList<DonationInfo> for DonationIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item=&'_ dyn Index<DonationInfo>> + '_> {
      let v: Vec<&dyn Index<DonationInfo>> = vec![&self.sender, &self.recipient];
      Box::new(v.into_iter())
    }
}

pub fn donation_sender_idx(d: &DonationInfo) -> AlpineUser { d.sender.clone() }

pub fn donation_recipient_idx(d: &DonationInfo) -> AlpineUser { d.recipient.clone() }
