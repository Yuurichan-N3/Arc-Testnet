pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const PAYME_CONTRACT: &str = "0x49197480afe1f6592324fa0a5ee389b4c3edc2b6";
const USDC: &str = "0x3600000000000000000000000000000000000000";
const RECIPIENT: &str = "0xab3f670120987d2592e98476c8a3b304c65956bc";
const PAY_AMOUNT: u64 = 100_050;

fn random_ref() -> [u8; 32] {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut seed = ts as u64;
    let mut out = [0u8; 32];
    for i in 0..4 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let bytes = seed.to_be_bytes();
        out[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }
    out
}

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Payme Arc category execution started");

    let payme = Address::from_str(PAYME_CONTRACT).unwrap();
    let usdc = Address::from_str(USDC).unwrap();
    let from = client.address();
    let amount = U256::from(PAY_AMOUNT);

    let approve_data = calldata::build_approve(payme, U256::MAX);
    match execute_tx(client, nonce, usdc, approve_data, U256::zero(), 100000, "Payme Arc Approve USDC").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Payme Arc Approve USDC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let ref_bytes = random_ref();
    match execute_tx(
        client,
        nonce,
        payme,
        calldata::build_pay(usdc, from, amount, ref_bytes),
        U256::zero(),
        300000,
        "Payme Arc Pay USDC",
    ).await {
        Ok(n) => {
            nonce = n;
            lg("Payme Arc payment completed");
        }
        Err(e) => lr(&format!("Payme Arc Pay USDC failed: {}", e)),
    }

    lg("Payme Arc category execution completed");
    Ok(nonce)
}