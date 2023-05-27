use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Incorrect Contract Name ({contract_name:?})")]
    IncorrectContractName { contract_name: String},
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Invalid Wallet Address ({address:?})")]
    InvalidWalletAddress { address: String },
    #[error("Address Already Registered")]
    UserAlreadyExists {  },
    #[error("Username Taken ({username:?})")]
    UsernameNotAvailable { username: String },
    #[error("User Not Found ({user:?})")]
    UserNotFound { user: String },
    #[error("Username cannot be empty")]
    EmptyUsername {},
    #[error("Invalid username ({username:?}) - {reason:?}")]
    InvalidUsername { username: String, reason: String },
    #[error("You must send a donation")]
    NoDonation {},
}
