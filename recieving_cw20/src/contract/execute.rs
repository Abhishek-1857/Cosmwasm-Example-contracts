use super::*;


////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//// Execute
////
//// This contract's Execute endpoint will be called directly by the
//// Cw20 contract itself <not by the user>
////
//// To get the Cw20 contract to do this, the user will need to call 
//// the "Send{contract, amount, msg}" execute on the Cw20 contract,
//// -> Where "contract" is the Address of this contract
//// -> Where "amount" is the amount of Cw20 tokens to send to this contract
//// -> Where "msg" is <in binary> the ReceiveMsg 
////~~~~
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // cw20 receive wrapper
        ExecuteMsg::Receive(receive_msg) => execute_receive(deps, env, info, receive_msg),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // Message included in Send{contract, amount, **msg**} execute on the cw20 contract
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;

    // Address that executed the "Send" on the cw20 contract
    let user_wallet = deps.api.addr_validate(&wrapper.sender)?;

    // Constructing cw20 balance
    let balance = Balance::Cw20(Cw20CoinVerified {
        // cw20 contract this message was sent from
        address: info.sender.clone(),
        // Send{contract, **amount**, msg}
        amount: wrapper.amount,
    });

    // Load config for whitelist check
    let config = CONFIG.load(deps.storage)?;

    // Check constructed cw20 balance , returns contract error if not
    is_balance_whitelisted(&balance, &config)?;

    match msg {
        // Message included in the "Send{contract, amount, **msg**}" call on the cw20 contract,
        ReceiveMsg::AnExecuteMsg {} => {
            execute_do_something(deps, &user_wallet, &info.sender, balance)
        }
    }
}

pub fn execute_do_something(
    _deps: DepsMut,
    _user_wallet: &Addr,
    _cw20_contract_addr: &Addr,
    _balance: Balance,
) -> Result<Response, ContractError> {
    // insert your execution logic here

    Ok(Response::default())
}
