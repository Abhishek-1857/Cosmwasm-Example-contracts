use super::*;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // admin info
    let admin = info.sender.to_string();
    let cw20_whitelist: Vec<(String, Addr)> = vec![(
        "COSMOS".to_string(),
        deps.api
            .addr_validate("cosmos168ctmpyppk90d34p3jjy658zf5a5l3w8wk35wht6ccqj4mr0yv8s4j5awr")?,
    )];

    CONFIG.save(
        deps.storage,
        &Config {
            admin: deps.api.addr_validate(&admin)?,
            cw20_wl: cw20_whitelist,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin))
}
