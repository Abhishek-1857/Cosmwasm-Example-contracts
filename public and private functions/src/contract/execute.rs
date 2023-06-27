use super::*;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::PublicFunction {param} => public_function(deps,env,info,param),
}
}


// Public function that can be called externally
pub fn public_function(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    param: String,
) -> Result<Response, ContractError> {
    // Perform the desired logic
    let result = do_something(param);

    // Return a response
    let response = Response::new().add_attribute("result", result);

    Ok(response)
}

// Private function that can only be called internally
fn do_something(param: String) -> String {
    // Perform some internal logic
    let result = format!("Doing something with param: {}", param);

    result
}
