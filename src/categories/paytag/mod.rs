pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, TransactionRequest, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const PAYTAG_CONTRACT: &str = "0x762C5be576CAFB489457b8426a7eF9C653328fbb";
const BRIDGE_CONTRACT: &str = "0xC5567a5E3370d4DBfB0540025078e283e36A363d";
const USDC: &str = "0x3600000000000000000000000000000000000000";

const BRIDGE_AMOUNT_ARB: u64 = 1_000;
const BRIDGE_AMOUNT_ETH: u64 = 1_000;
const BRIDGE_AMOUNT_BASE: u64 = 100;
const BRIDGE_NONCE: u32 = 1000;

const DEST_ARB_SEPOLIA: u32 = 3;
const DEST_ETH_SEPOLIA: u32 = 0;
const DEST_BASE_SEPOLIA: u32 = 6;

const USERNAME_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const USERNAME_LEN: usize = 10;

fn random_username() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut seed = ts as u64;
    let mut result = String::with_capacity(USERNAME_LEN);
    for _ in 0..USERNAME_LEN {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let idx = (seed as usize) % USERNAME_CHARS.len();
        result.push(USERNAME_CHARS[idx] as char);
    }
    result
}

async fn is_already_registered(client: &Client) -> bool {
    let contract: Address = PAYTAG_CONTRACT.parse().unwrap();
    let username = random_username();
    let data = calldata::build_register_username(&username);
    let tx = TransactionRequest::new()
        .from(client.address())
        .to(contract)
        .data(data)
        .value(U256::zero());
    match client.call(&tx.into(), None).await {
        Ok(_) => false,
        Err(_) => true,
    }
}

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Paytag category execution started");

    let paytag = Address::from_str(PAYTAG_CONTRACT).unwrap();
    let bridge = Address::from_str(BRIDGE_CONTRACT).unwrap();
    let usdc = Address::from_str(USDC).unwrap();
    let recipient = client.address();

    if is_already_registered(client).await {
        ly("Paytag username already registered for this wallet, skipping registration");
    } else {
        let username = random_username();
        ly(&format!("Paytag registering username for this wallet"));
        match execute_tx(
            client,
            nonce,
            paytag,
            calldata::build_register_username(&username),
            U256::zero(),
            200000,
            "Paytag Register Username",
        )
        .await
        {
            Ok(n) => {
                nonce = n;
                lg("Paytag username registration completed");
            }
            Err(e) => lr(&format!("Paytag Register Username failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    }

    let allowance_arb = calldata::build_increase_allowance(bridge, U256::from(BRIDGE_AMOUNT_ARB));
    match execute_tx(client, nonce, usdc, allowance_arb, U256::zero(), 100000, "Paytag Bridge ARB Allowance").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Paytag Bridge ARB allowance failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client,
        nonce,
        bridge,
        calldata::build_bridge(
            U256::from(BRIDGE_AMOUNT_ARB),
            recipient,
            usdc,
            bridge,
            DEST_ARB_SEPOLIA,
            BRIDGE_NONCE,
        ),
        U256::zero(),
        400000,
        "Paytag Bridge USDC to Arbitrum Sepolia",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Paytag bridge to Arbitrum Sepolia completed");
        }
        Err(e) => lr(&format!("Paytag Bridge USDC to Arbitrum Sepolia failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let allowance_eth = calldata::build_increase_allowance(bridge, U256::from(BRIDGE_AMOUNT_ETH));
    match execute_tx(client, nonce, usdc, allowance_eth, U256::zero(), 100000, "Paytag Bridge ETH Allowance").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Paytag Bridge ETH allowance failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client,
        nonce,
        bridge,
        calldata::build_bridge(
            U256::from(BRIDGE_AMOUNT_ETH),
            recipient,
            usdc,
            bridge,
            DEST_ETH_SEPOLIA,
            BRIDGE_NONCE,
        ),
        U256::zero(),
        400000,
        "Paytag Bridge USDC to ETH Sepolia",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Paytag bridge to ETH Sepolia completed");
        }
        Err(e) => lr(&format!("Paytag Bridge USDC to ETH Sepolia failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let allowance_base = calldata::build_increase_allowance(bridge, U256::from(BRIDGE_AMOUNT_BASE));
    match execute_tx(client, nonce, usdc, allowance_base, U256::zero(), 100000, "Paytag Bridge Base Allowance").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Paytag Bridge Base allowance failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client,
        nonce,
        bridge,
        calldata::build_bridge(
            U256::from(BRIDGE_AMOUNT_BASE),
            recipient,
            usdc,
            bridge,
            DEST_BASE_SEPOLIA,
            BRIDGE_NONCE,
        ),
        U256::zero(),
        400000,
        "Paytag Bridge USDC to Base Sepolia",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Paytag bridge to Base Sepolia completed");
        }
        Err(e) => lr(&format!("Paytag Bridge USDC to Base Sepolia failed: {}", e)),
    }

    lg("Paytag category execution completed");
    Ok(nonce)
}