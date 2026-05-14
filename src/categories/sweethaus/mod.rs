pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::config::SweethausConfig;
use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const SWEETHAUS_CONTRACT: &str = "0x55A63956f4d56958C8C2149A66FE3693DEF4A6C7";

const MINT_VALUE_WEI: u64 = 0xb7bc7e60fd6a;

pub async fn run(client: &Client, mut nonce: U256, _cfg: &SweethausConfig) -> Result<U256> {
    ly("Sweet Haus category execution started");

    let contract = Address::from_str(SWEETHAUS_CONTRACT).unwrap();
    let wallet_addr = client.address();


    let data = calldata::build_claim(wallet_addr);

    match execute_tx(
        client,
        nonce,
        contract,
        data,
        U256::from(MINT_VALUE_WEI),
        300000,
        "Sweet Haus Mint NFT",
    )
    .await
    {
        Ok(next_nonce) => {
            nonce = next_nonce;
            lg("Sweet Haus NFT mint completed successfully");
        }
        Err(e) => {
            lr(&format!("Sweet Haus NFT mint failed: {}", e));
            return Err(e);
        }
    }

    Ok(nonce)
}