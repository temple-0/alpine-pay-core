mod error;
pub mod msg;
pub mod state;
pub mod traits;
pub mod execute;
pub mod query;
pub mod contract_tests;

pub use crate::error::ContractError;
pub use crate::msg::{
    InstantiateMsg,
    MigrateMsg,
    ExecuteMsg,
    QueryMsg
};
pub use crate::state::AlpineContract;

// allow users to interact via external data by sending instantiate/execute/query message
#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let contract = AlpineContract::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: MigrateMsg
    ) -> Result<Response, ContractError> {
        let contract = AlpineContract::default();
        contract.migrate(deps, env, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg
    ) -> Result<Response, ContractError> {
        let contract = AlpineContract::default();
        contract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let contract = AlpineContract::default();
        contract.query(deps, env, msg)
    }
}