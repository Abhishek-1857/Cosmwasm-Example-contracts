use super::*;

#[cw_serde]
pub struct OperationsResponse {
    pub addition_result: Uint256,
    pub subtraction_result: Uint256,
    pub multiplication_result: Uint256,
    pub division_result: Uint256,
    pub modulo_result: Uint256,
    pub exponentiation_result: Uint256,
}
// Mapping of result of two numbers

pub const RESULT: Item<OperationsResponse> = Item::new("result");