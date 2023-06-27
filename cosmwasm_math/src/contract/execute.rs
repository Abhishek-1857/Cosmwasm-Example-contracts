use super::*;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::Operations { a, b } => execute_operations(deps,a,b),
}
}

fn execute_operations(
    deps: DepsMut,
    a: Uint256,
    b: Uint256
) -> Result<Response,ContractError>{

    // Checking if numbers are not zero
    if a.is_zero() && b.is_zero() {
        return Err(ContractError::CanNotBeZero());
    }

     // Addition
     let addition_result = a + b;

     // Subtraction
     let subtraction_result = a - b;
 
     // Multiplication
     let multiplication_result = a * b;
 
     // Division
     let division_result = a / b;
 
     // Modulo
     let modulo_result = a % b;
 
     // Exponentiation
     let exponent: u32 = 3;
     let exponentiation_result: Uint256 = a.pow(exponent);

     // Create the response
    let response = OperationsResponse {
        addition_result,
        subtraction_result,
        multiplication_result,
        division_result,
        modulo_result,
        exponentiation_result,
    };

    // Fetching the state
      RESULT.load(deps.storage).unwrap();

    // Update the state
    RESULT.save(deps.storage, &response).unwrap();

    let res = Response::new()
    .add_attributes(vec![
        ("action", "operations"),
        ("a", &a.to_string()),
        ("b", &b.to_string()),
        ("addition_res",&addition_result.to_string()),
        ("substraction_res",&subtraction_result.to_string()),
        ("multiplicationn_res",&multiplication_result.to_string()),
        ("division_res",&division_result.to_string()),
        ("modulo_res",&modulo_result.to_string()),
        ("exponential_res",&exponentiation_result.to_string()),
    ]);

    Ok(res)
}