pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const CONTRACT: &str = "0x83f83591C57D5E21da3A3e69d7958C07FcC87F9B";
const MINT_VALUE: u64 = 200_000_000_000_000_000;

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Omnihub category execution started");

    let contract = Address::from_str(CONTRACT).unwrap();

    match execute_tx(
        client,
        nonce,
        contract,
        calldata::build_mint_shrimp(1),
        U256::from(MINT_VALUE),
        300000,
        "Omnihub Mint Shrimp",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Omnihub Mint Shrimp completed");
        }
        Err(e) => lr(&format!("Omnihub Mint Shrimp failed: {}", e)),
    }

    Ok(nonce)
}