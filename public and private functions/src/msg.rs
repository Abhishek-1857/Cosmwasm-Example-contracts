use super::*;


#[cw_serde]
pub struct InstantiateMsg {}


#[cw_serde]
pub enum ExecuteMsg {

    PublicFunction { param: String }
    
}
