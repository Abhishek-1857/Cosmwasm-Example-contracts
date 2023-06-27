#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;


use cw20::{Cw20Coin,Cw20ReceiveMsg};