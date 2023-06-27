#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

use crate::{error::ContractError, msg::*, state::*};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse,
    Response, StdError,
};
use cw20::{Balance, Cw20CoinVerified, Cw20ReceiveMsg};
use thiserror::Error;

use cw_storage_plus::Item;
