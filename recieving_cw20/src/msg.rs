use super::*;

///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Instantiate
///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
pub struct InstantiateMsg {}

///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Execute
///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
pub enum ExecuteMsg {
    // Receive Filter
    Receive(Cw20ReceiveMsg),
}

////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Message(s) from cw20 contract
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
pub enum ReceiveMsg {
    AnExecuteMsg {},
}

///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Query
///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AdminResponse)]
    GetAdmin {},
}

////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Query Response
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}
