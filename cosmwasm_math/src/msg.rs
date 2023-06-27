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
    
    Operations {
        a: Uint256,
        b: Uint256,
    },
}


///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////// Query
///~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint256)]
    GetResponse {},
}


