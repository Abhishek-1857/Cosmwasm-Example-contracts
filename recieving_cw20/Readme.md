## RECIEVE CW20 TOKENS IN YOUR CONTRACT

This is a  example of a CosmWasm contract that implements the [Cw20 Receiver Interface](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md#receiver)

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

This will deploy the contract and will give contract address which will be used to send cw20 token to.

#### Rest of the workflow

This example contract's Execute endpoint will be called directly by the Cw20 contract itself not by the user

To get the Cw20 contract to do this, the user will need to call the `Send{contract, amount, msg}` execute on the Cw20 contract,
- Where `contract` is the Address of this contract
- Where `amount` is the amount of Cw20 tokens to send to this contract
- Where `msg` is `in binary` the ReceiveMsg of our contract