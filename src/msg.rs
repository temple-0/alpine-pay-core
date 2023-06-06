use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{DonationInfo, AlpineUser};
use cosmwasm_std::{Addr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendDonation { sender: String, recipient: String, message: String },
    RegisterUser { user: AlpineUser, username: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetSentDonations{ sender: String },
    GetReceivedDonations { recipient: String },
    GetDonationCount {  },
    IsUsernameAvailable { username: String },
    GetAllUsers { },
    GetUserByAddr { address: Addr },
    GetUserByName { username: String }
}

// Return a list of donation IDs mapped to the data stored in the donation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiDonationResponse{
    pub donations: Vec<(Vec<u8>, DonationInfo)>
}

// Return a list of Alpine users
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiUserResponse{
    pub users: Vec<AlpineUser>
}

// Return the count of all donations in the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DonationCountResponse{
    pub count: u64
}

// Return whether the queried username is available
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsernameAvailableResponse {
    pub is_available: bool
}

// Returns a single Alpine user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AlpineUserResponse{
    pub user: AlpineUser,
}