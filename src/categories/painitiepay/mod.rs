pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const USDC: &str = "0x3600000000000000000000000000000000000000";
const RECEIVER: &str = "0xAb3F670120987D2592e98476c8a3b304c65956Bc";
const TRANSFER_AMOUNT: u64 = 1_000_000;

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Painitiepay category execution started");

    let usdc = Address::from_str(USDC).unwrap();
    let receiver = Address::from_str(RECEIVER).unwrap();

    match execute_tx(
        client,
        nonce,
        usdc,
        calldata::build_transfer(receiver, U256::from(TRANSFER_AMOUNT)),
        U256::zero(),
        100000,
        "Painitiepay Pay 1 USDC",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Painitiepay Pay completed");
        }
        Err(e) => lr(&format!("Painitiepay Pay 1 USDC failed: {}", e)),
    }

    Ok(nonce)
}