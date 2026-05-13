pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::constants::{WATCHOOR_CONTRACT, WATCHOOR_VALUE_WEI};
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, Client};

pub async fn run(client: &Client, mut nonce: U256, times: u32) -> Result<U256> {
    ly("Watchoor category execution started");

    let contract = Address::from_str(WATCHOOR_CONTRACT).unwrap();
    let value = U256::from(WATCHOOR_VALUE_WEI);

    for _ in 0..times {
        nonce = execute_tx(
            client, nonce, contract,
            calldata::build_gm(),
            value, 140000, "Good Morning",
        )
        .await
        .map_err(|e| { lr(&format!("Watchoor Good Morning failed: {}", e)); e })?;

        nonce = execute_tx(
            client, nonce, contract,
            calldata::build_gn(),
            value, 140000, "Good Night",
        )
        .await
        .map_err(|e| { lr(&format!("Watchoor Good Night failed: {}", e)); e })?;

        nonce = execute_tx(
            client, nonce, contract,
            calldata::build_deploy_nft("Txguazu", "TXGU"),
            value, 240000, "Deploy NFT",
        )
        .await
        .map_err(|e| { lr(&format!("Watchoor Deploy NFT failed: {}", e)); e })?;

        nonce = execute_tx(
            client, nonce, contract,
            calldata::build_deploy_erc20("Txguazu", "TXGU"),
            value, 240000, "Deploy ERC20",
        )
        .await
        .map_err(|e| { lr(&format!("Watchoor Deploy ERC20 failed: {}", e)); e })?;

        nonce = execute_tx(
            client, nonce, contract,
            calldata::build_deploy_counter(),
            value, 160000, "Deploy Counter",
        )
        .await
        .map_err(|e| { lr(&format!("Watchoor Deploy Counter failed: {}", e)); e })?;
    }

    Ok(nonce)
}
