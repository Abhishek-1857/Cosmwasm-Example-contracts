use super::*;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetAdmin {} => get_admin(deps),
    }
}

pub fn get_admin(deps: Deps) -> Result<QueryResponse, StdError> {
    let config = CONFIG.load(deps.storage)?;

    let admin = config.admin.to_string();

    to_binary(&AdminResponse { admin })
}
