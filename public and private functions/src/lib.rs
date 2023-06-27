#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;


use crate::{error::ContractError, msg::*};
use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo,
    Response
};
use thiserror::Error;