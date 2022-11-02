#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
// use cw2::set_contract_version;

//use crate::debug::print;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{NUMBER, NUMBER2};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flow-test-2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Flow {} => execute_flow(deps, env),
        ExecuteMsg::Flow2 {} => execute_flow2(deps, env),
    }
}

pub fn execute_flow(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let value = NUMBER.load(deps.storage).unwrap_or(0);
    NUMBER.save(deps.storage, &(value + 1))?;
    let value = NUMBER.load(deps.storage).unwrap_or(0);
    if value < 20 {
        if value % 3 == 0 {
            let msgs1 = to_binary(&ExecuteMsg::Flow2 {})?;
            let msgs2 = to_binary(&ExecuteMsg::Flow {})?;
            Ok(Response::new()
                .add_attribute("action", "into flow2")
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: env.contract.address.to_string(),
                    msg: msgs1,
                    funds: vec![],
                }))
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: env.contract.address.to_string(),
                    msg: msgs2,
                    funds: vec![], 
                })))
        } else {
            let msgs1 = to_binary(&ExecuteMsg::Flow {})?;
            Ok(Response::new()
                .add_attribute("action", value.to_string())
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: env.contract.address.to_string(),
                    msg: msgs1,
                    funds: vec![],
                })))
        }
    } else {
        Ok(Response::new().add_attribute("state", "finish"))
    }
}

pub fn execute_flow2(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let value = NUMBER2.load(deps.storage).unwrap_or(0);
    if value < 3 {
        let msgs1 = to_binary(&ExecuteMsg::Flow2 {})?;
        let mut s = String::from(NUMBER.load(deps.storage)?.to_string());
        s.push_str("-");
        s.push_str(&value.to_string());
        Ok(Response::new()
            .add_attribute("action", s)
            .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: env.contract.address.to_string(),
                msg: msgs1,
                funds: vec![],
            })))
    } else {
        let mut s = String::from(NUMBER.load(deps.storage)?.to_string());
        s.push_str("-3");
        NUMBER2.save(deps.storage, &(0))?;
        Ok(Response::new().add_attribute("action", s))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
