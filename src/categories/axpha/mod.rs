pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::AxphaConfig;
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, Client};

const CONTRACT: &str = "0xe9f3e9a6c9304d384c38ad8db5b4707580e8714c";

const TOKEN_EURC: &str = "0x89b50855aa3be2f677cd6303cec089b5f319d72a";
const TOKEN_AD: &str = "0x808e4e5a6006296b274c02683d17047bea92e6ba";
const TOKEN_CIRCLE: &str = "0xe8bc5d6c5bd36b1984b54a5b593f61ae668acc27";

const VALUE_WEI: u64 = 100_000_000_000_000_000;
const DEADLINE_SECS: u64 = 3600;
const GAS_LIMIT: u64 = 520_000;

pub async fn run(client: &Client, mut nonce: U256, cfg: &AxphaConfig) -> Result<U256> {
    ly("Axpha category execution started");

    let contract = Address::from_str(CONTRACT).unwrap();
    let recipient = client.address();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let deadline = now + DEADLINE_SECS;

    let mapping: &[(&str, &str, Address)] = &[
        ("usdc_to_eurc", "Axpha Swap USDC to EURC", Address::from_str(TOKEN_EURC).unwrap()),
        ("usdc_to_ad",   "Axpha Swap USDC to AD",   Address::from_str(TOKEN_AD).unwrap()),
        ("usdc_to_circle", "Axpha Swap USDC to CIRCLE", Address::from_str(TOKEN_CIRCLE).unwrap()),
    ];

    let swaps = match &cfg.swaps {
        Some(s) => s,
        None => return Ok(nonce),
    };

    for (key, label, token_out) in mapping {
        let count = match *key {
            "usdc_to_eurc"   => swaps.usdc_to_eurc.unwrap_or(0),
            "usdc_to_ad"     => swaps.usdc_to_ad.unwrap_or(0),
            "usdc_to_circle" => swaps.usdc_to_circle.unwrap_or(0),
            _ => 0,
        };

        for _ in 0..count {
            let data = calldata::build_swap(*token_out, recipient, deadline);

            nonce = execute_tx(
                client, nonce, contract,
                data,
                U256::from(VALUE_WEI), GAS_LIMIT, label,
            )
            .await
            .map_err(|e| { lr(&format!("{} failed: {}", label, e)); e })?;
        }
    }

    Ok(nonce)
}
