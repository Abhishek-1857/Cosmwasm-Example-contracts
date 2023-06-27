## COSMWASM MATHS OPERATIONS

This is a  example of a CosmWasm contract that implements simple maths operations like addition, substraction, multiplication, division, modulo and exponential

---

### Working

#### Instantiate

To understand the working of this example first instantiate the contract using the function `instantiate` in `init.rs`.

```rust
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError>
```

This will deploy the contract and will give contract address which will be used to interact with the contract.

#### Execute

This example contract's Execute endpoint will be called by any user who want to perform mathamatical operations

```rust
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
```
This `execute` function takes a enum of `ExecuteMsg` which actually contains all the contract function and matches them with the function user is calling. In our case `Operations`. Then it calls `execute_operations` function:

```rust
fn execute_operations(
    deps: DepsMut,
    a: Uint256,
    b: Uint256
) -> Result<Response,ContractError>
```

This function takes two parameters a and b for mathmatical operations and store the result in `RESULT` global state variable stored in `state.rs` :

```rust
pub const RESULT: Item<OperationsResponse> = Item::new("result");
```

***NOTE  We are using `Item` here for storage if we want better storage options then we can use `MAP` from `cw_storage_plus` which store values in key-value pairs.
We can query the result using next step.

#### Query

This example query endpoint is basically getting the result  of mathmatical operations  .

```rust
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetResponse {} => get_response(deps),
    }
}
```
This `query` function takes a enum of `QueryMsg` which actually contains all the contract query function and matches them with the function user is calling. In our case `GetResponse`. Then it calls `get_response` function:

```rust
pub fn get_response(deps: Deps) -> Result<QueryResponse, StdError> {
    let result = RESULT.load(deps.storage)?;

    to_binary(&result)
}
```
 This function return the result of our mathmatical operation.

---

