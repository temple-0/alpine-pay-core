#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    ensure_eq,
    DepsMut, 
    Env, 
    MessageInfo, 
    Response,
    BankMsg
};
use cw2::{
    set_contract_version,
    get_contract_version
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    InstantiateMsg,
    MigrateMsg
};
use crate::state::{
    AlpineContract,
    DonationInfo,
    AlpineUser
};
use crate::traits::{
    DonationExecute
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:alpine-superchats";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> AlpineContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default())
    }

    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg
    ) -> Result<Response, ContractError> {
        let ver = get_contract_version(deps.storage)?;
        ensure_eq!(ver.contract, CONTRACT_NAME, ContractError::IncorrectContractName { contract_name: String::from(CONTRACT_NAME) });
        set_contract_version(deps.storage, ver.contract, ver.version.clone())?;
        
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SendDonation { sender, recipient, message } => self.send_donation(deps, _env, info, sender, recipient, message),
            ExecuteMsg::RegisterUser { user, username } => {
                if info.sender != user.address {
                    return Err(ContractError::InvalidWalletAddress { address: user.address.to_string() })
                }
                self.register_user(deps, _env, user, username)
            }
        }
    }
}

impl<'a> DonationExecute for AlpineContract<'a> {
    fn send_donation(
        &self,
        deps: DepsMut, 
        _env: Env, 
        info: MessageInfo,
        sender: String,
        recipient: String, 
        message: String
    ) -> Result<Response, ContractError> {
        if recipient.is_empty() {
            return Err(ContractError::EmptyUsername {})
        }

        if info.funds.is_empty() {
            return Err(ContractError::NoDonation{})
        }

        let sender_user = match sender.is_empty() {
            true => AlpineUser::new(deps.as_ref(), info.sender.clone(), None)?,
            false => self.find_alpine_username(deps.storage, sender)?
        };

        if info.sender != sender_user.address {
            return Err(ContractError::InvalidWalletAddress { address: sender_user.address.to_string() })
        }

        let recipient_user = self.find_alpine_username(deps.storage, recipient)?;

        let donation = DonationInfo {
            sender: sender_user,
            recipient: recipient_user,
            amount: info.funds,
            message: message,
            timestamp: Some(_env.block.time)
        };

        let id = self.increment_donations(deps.storage)?;
        self.donations.update(deps.storage, &id.to_string(), |old| match old {
            Some(_) => Err(ContractError::Unauthorized {}),
            None => Ok(donation.clone())
        })?;

        let bankmsg = BankMsg::Send {
            to_address: donation.recipient.address.to_string(),
            amount: donation.amount.clone()
        };

        Ok(Response::new().add_message(bankmsg))
    }

    fn register_user(
        &self,
        deps: DepsMut,
        _env: Env,
        mut user: AlpineUser,
        username: String
    ) -> Result<Response, ContractError> {
        
        let valid_username = match validate_username(username.clone()) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        user = match user.username.is_empty() {
            true => {
                match self.get_user_by_address(deps.storage, user.address.clone()) {
                    Ok(_) => {
                        return Err(ContractError::UserNotFound { user: user.address.clone().to_string() })
                    },
                    Err(_) => AlpineUser::new(deps.as_ref(), user.address.clone(), None)?
                }
            },
            false => return Err(ContractError::UserAlreadyExists {  } )
        };

        let searched_username = match self.usernames.may_load(deps.storage, valid_username.clone()) {
            Ok(result) => match result {
                Some(_) => Err(ContractError::UsernameNotAvailable { username: valid_username.clone() }),
                None => Ok(valid_username.clone())
            },
            Err(e) => Err(ContractError::Std(e))
        }?;

        user.username = searched_username;
        self.usernames.save(deps.storage, username, &user)?;
        self.addresses.save(deps.storage, user.address.clone(), &user)?;
        
        Ok(Response::new().add_attribute("username", user.username))
    }
}

fn validate_username(username: String) -> Result<String, ContractError> {
    if username.is_empty() {
        return Err(ContractError::EmptyUsername {})
    }

    if username.len() > 32 {
        return Err(ContractError::InvalidUsername { 
            username,
            reason: String::from("must be shorter than 33 characters")
        })
    }

    for c in username.chars() {
        if !(c.is_ascii_alphabetic() || c.is_numeric() || c == '-' || c == '_') {
            return Err(ContractError::InvalidUsername { 
                username,
                reason: String::from("only alphanumeric, underscores, and dashes are allowed")
            })
        }
    }

    Ok(username)
}
