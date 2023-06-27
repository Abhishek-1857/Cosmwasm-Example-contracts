use super::*;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // updation result state to zero

    RESULT.save(
        deps.storage,
        &&OperationsResponse { 
            addition_result: Uint256::zero(), 
            subtraction_result: Uint256::zero(), 
            multiplication_result: Uint256::zero(), 
            division_result: Uint256::zero(), 
            modulo_result: Uint256::zero(), 
            exponentiation_result: Uint256::zero() }
    ).unwrap();

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
    
}
