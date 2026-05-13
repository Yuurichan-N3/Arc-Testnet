pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::constants::{SUPERBRIDGE_AMOUNT, TOKEN_MESSENGER_ARC, USDC_ARC_TOKEN};
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, silent_approve, Client};

pub async fn run(client: &Client, mut nonce: U256, times: u32) -> Result<U256> {
    ly("Super bridge category execution started");

    let messenger = Address::from_str(TOKEN_MESSENGER_ARC).unwrap();
    let usdc = Address::from_str(USDC_ARC_TOKEN).unwrap();
    let wallet_addr = client.address();

    for _ in 0..times {
        nonce = silent_approve(
            client, nonce, usdc, messenger,
            U256::from(SUPERBRIDGE_AMOUNT),
        )
        .await
        .map_err(|e| { lr(&format!("Super bridge approve failed: {}", e)); e })?;

        nonce = execute_tx(
            client, nonce, messenger,
            calldata::build_deposit_for_burn(wallet_addr),
            U256::zero(), 380000, "Bridge burn deposit",
        )
        .await
        .map_err(|e| { lr(&format!("Super bridge deposit failed: {}", e)); e })?;
    }

    Ok(nonce)
}
