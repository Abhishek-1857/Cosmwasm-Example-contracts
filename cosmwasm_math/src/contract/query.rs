use super::*;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetResponse {} => get_response(deps),
    }
}

pub fn get_response(deps: Deps) -> Result<QueryResponse, StdError> {
    let result = RESULT.load(deps.storage)?;

    to_binary(&result)
}
