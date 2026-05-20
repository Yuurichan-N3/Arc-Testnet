pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, TransactionRequest, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const CONTRACT: &str = "0xB22d08cFCeE07Fc46c0840C40c10712cA9299f81";

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

async fn already_done(client: &Client, data: ethers::types::Bytes) -> bool {
    let contract: Address = CONTRACT.parse().unwrap();
    let tx = TransactionRequest::new()
        .from(client.address())
        .to(contract)
        .data(data)
        .value(U256::zero());
    client.call(&tx.into(), None).await.is_err()
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Inuxarc category execution started");

    let contract = Address::from_str(CONTRACT).unwrap();

    if already_done(client, calldata::build_gm()).await {
        ly("Inuxarc GM already said today, skipping");
    } else {
        match execute_tx(client, nonce, contract, calldata::build_gm(), U256::zero(), 200000, "Inuxarc Say GM").await {
            Ok(n) => { nonce = n; lg("Inuxarc say GM completed"); }
            Err(e) => lr(&format!("Inuxarc Say GM failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    }

    if already_done(client, calldata::build_gn()).await {
        ly("Inuxarc GN already said today, skipping");
    } else {
        match execute_tx(client, nonce, contract, calldata::build_gn(), U256::zero(), 200000, "Inuxarc Say GN").await {
            Ok(n) => { nonce = n; lg("Inuxarc say GN completed"); }
            Err(e) => lr(&format!("Inuxarc Say GN failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    }

    lg("Inuxarc category execution completed");
    Ok(nonce)
}