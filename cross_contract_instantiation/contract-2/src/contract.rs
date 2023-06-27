#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response,CosmosMsg};


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Instantiate {} => try_instantiate(deps,info),
    }
}

pub fn try_instantiate(_deps: DepsMut,info: MessageInfo) -> Result<Response, ContractError> {

   // Creating instantiate message of contract 1 from this contract
   let instantiation_msg=contract_1::msg::InstantiateMsg{ count: 1 };

   // Instantiating the contract 1 
   let msgs=CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate { 
       admin: None, 
       code_id: 0, 
       msg: to_binary(&instantiation_msg)?, 
       funds: info.funds, 
       label: "Cross_Contract_instantiation".to_string() });


   let res=    Response::new()
   .add_attribute("method", "instantiate_another").add_message(msgs);

    Ok(res)
}


