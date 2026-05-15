pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const CONTRACT: &str = "0xaE933dE72586F4dA6be93C64D99fB702d3a34200";
const DEPOSIT_VALUE: u64 = 10_000_000_000_000_000;

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Flow Three category execution started");

    let contract = Address::from_str(CONTRACT).unwrap();
    let deposit_value = U256::from(DEPOSIT_VALUE);

    match execute_tx(
        client,
        nonce,
        contract,
        calldata::build_deposit(),
        deposit_value,
        50000,
        "Flow Three Deposit",
    )
    .await
    {
        Ok(n) => nonce = n,
        Err(e) => {
            lr(&format!("Flow Three deposit failed: {}", e));
            return Ok(nonce);
        }
    }

    match execute_tx(
        client,
        nonce,
        contract,
        calldata::build_withdraw(deposit_value),
        U256::zero(),
        50000,
        "Flow Three Withdraw",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Flow Three deposit and withdraw completed");
        }
        Err(e) => lr(&format!("Flow Three withdraw failed: {}", e)),
    }

    Ok(nonce)
}