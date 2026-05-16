pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::PrestodexConfig;
use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const ROUTER: &str = "0x5794a8284A29493871Fbfa3c4f343D42001424D6";
const USDC: &str = "0x3600000000000000000000000000000000000000";
const EURC: &str = "0x89B50855Aa3bE2F677cD6303Cec089B5F319D72a";
const USYC: &str = "0x825Ae482558415310C71B7E03d2BbBe409345903";
const BRIDGE_CONTRACT: &str = "0xC5567a5E3370d4DBfB0540025078e283e36A363d";

const SWAP_AMOUNT_IN: u64 = 100_000;
const SWAP_MIN_OUT: u64 = 1;

const LP_EURC_USDC_A: u64 = 100;
const LP_USYC_USDC_A: u64 = 1;
const LP_USDC_ALLOWANCE: u64 = 10_000;

const BRIDGE_AMOUNT: u64 = 210_000;
const BRIDGE_SLIPPAGE_BPS: u64 = 300;
const BRIDGE_DEST_CHAIN_ID: u32 = 6;

fn deadline() -> U256 {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    U256::from(ts + 600)
}

fn now_u256() -> U256 {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    U256::from(ts)
}

fn bridge_min_amount(amount: u64) -> U256 {
    let min = amount * (10_000 - BRIDGE_SLIPPAGE_BPS) / 10_000;
    U256::from(min)
}

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

pub async fn run(client: &Client, mut nonce: U256, cfg: &PrestodexConfig) -> Result<U256> {
    ly("Presto Dex category execution started");

    let router = Address::from_str(ROUTER).unwrap();
    let usdc = Address::from_str(USDC).unwrap();
    let eurc = Address::from_str(EURC).unwrap();
    let usyc = Address::from_str(USYC).unwrap();
    let bridge_contract = Address::from_str(BRIDGE_CONTRACT).unwrap();

    if let Some(swap_cfg) = &cfg.swap {
        if swap_cfg.enabled {
            let approve_data = calldata::build_increase_allowance(router, U256::from(SWAP_AMOUNT_IN));
            match execute_tx(client, nonce, usdc, approve_data, U256::zero(), 100000, "Presto Dex Swap USDC Allowance").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex approve USDC for swap failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;

            match execute_tx(
                client,
                nonce,
                router,
                calldata::build_swap(
                    usdc,
                    usyc,
                    U256::from(SWAP_AMOUNT_IN),
                    U256::from(SWAP_MIN_OUT),
                    now_u256(),
                ),
                U256::zero(),
                300000,
                "Presto Dex Swap USDC to USYC",
            )
            .await
            {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex Swap USDC to USYC failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;
            lg("Presto Dex swap execution completed");
        }
    }

    if let Some(lp_cfg) = &cfg.add_lp {
        if lp_cfg.enabled {
\            let approve_eurc = calldata::build_approve(router, U256::from(LP_EURC_USDC_A * 10));
            match execute_tx(client, nonce, eurc, approve_eurc, U256::zero(), 100000, "Presto Dex LP EURC Allowance").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex approve EURC for LP failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;

\            let approve_usdc_lp = calldata::build_increase_allowance(router, U256::from(LP_USDC_ALLOWANCE));
            match execute_tx(client, nonce, usdc, approve_usdc_lp, U256::zero(), 100000, "Presto Dex LP USDC Allowance").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex approve USDC for LP failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;

            match execute_tx(
                client,
                nonce,
                router,
                calldata::build_add_liquidity(
                    eurc,
                    usdc,
                    U256::from(LP_EURC_USDC_A),
                    deadline(),
                ),
                U256::zero(),
                350000,
                "Presto Dex Add LP EURC/USDC",
            )
            .await
            {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex Add LP EURC/USDC failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;

            let approve_usyc = calldata::build_approve(router, U256::from(LP_USYC_USDC_A * 10));
            match execute_tx(client, nonce, usyc, approve_usyc, U256::zero(), 100000, "Presto Dex LP USYC Allowance").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Presto Dex approve USYC for LP failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;

            match execute_tx(
                client,
                nonce,
                router,
                calldata::build_add_liquidity(
                    usyc,
                    usdc,
                    U256::from(LP_USYC_USDC_A),
                    deadline(),
                ),
                U256::zero(),
                350000,
                "Presto Dex Add LP USYC/USDC",
            )
            .await
            {
                Ok(n) => {
                    nonce = n;
                    lg("Presto Dex add liquidity execution completed");
                }
                Err(e) => lr(&format!("Presto Dex Add LP USYC/USDC failed: {}", e)),
            }

            nonce = refresh_nonce(client, nonce).await;
        }
    }

    if let Some(bridge_cfg) = &cfg.bridge {
        if bridge_cfg.enabled {
            let bridge_amount = U256::from(BRIDGE_AMOUNT);
            let min_amount = bridge_min_amount(BRIDGE_AMOUNT);
            let recipient = client.address();
            let bridge_nonce = (now_u256().as_u64() % 100_000) as u32;

            let allowance_data = calldata::build_increase_allowance(bridge_contract, bridge_amount);
            match execute_tx(client, nonce, usdc, allowance_data, U256::zero(), 100000, "Presto Dex Bridge USDC Allowance").await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Presto Dex Bridge USDC allowance failed: {}", e));
                    return Ok(nonce);
                }
            }

            nonce = refresh_nonce(client, nonce).await;

            match execute_tx(
                client,
                nonce,
                bridge_contract,
                calldata::build_bridge(
                    bridge_amount,
                    min_amount,
                    recipient,
                    usdc,
                    bridge_contract,
                    BRIDGE_DEST_CHAIN_ID,
                    bridge_nonce,
                ),
                U256::zero(),
                400000,
                "Presto Dex Bridge USDC to Base Sepolia",
            )
            .await
            {
                Ok(n) => {
                    nonce = n;
                    lg("Presto Dex bridge execution completed");
                }
                Err(e) => lr(&format!("Presto Dex Bridge USDC to Base Sepolia failed: {}", e)),
            }
        }
    }

    Ok(nonce)
}