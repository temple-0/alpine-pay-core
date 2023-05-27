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
    // todo: add something here to get the number of donations sent
    GetSentDonations{ sender: String },
    GetReceivedDonations { recipient: String },
    GetSingleDonation{ id: u64 },
    IsUsernameAvailable { username: String },
    GetAllUsers { },
    GetUserByAddr { address: Addr },
    GetUserByName { username: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiDonationResponse{
    pub donations: Vec<(Vec<u8>, DonationInfo)>
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiUserResponse{
    pub users: Vec<AlpineUser>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SingleDonationResponse{
    pub donation: DonationInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct HostResponse{
    pub host: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NumDonationsResponse{
    pub count: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsernameAvailableResponse {
    pub is_available: bool
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AlpineUserResponse{
    pub user: AlpineUser,
}