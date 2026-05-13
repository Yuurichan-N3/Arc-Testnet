pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, Bytes, U256};
use std::str::FromStr;

use crate::config::CurvedexConfig;
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, preflight_check, silent_approve, Client};

const DEX_CONTRACT: &str = "0xff5cb29241f002ffed2eaa224e3e996d24a6e8d1";
const POOL_USDC_EURC: &str = "0x2d84d79c852f6842abe0304b70bbaa1506add457";
const STAKE_CONTRACT: &str = "0xcd4e6c8056608e7ca5b8cd126f32c56c43d92979";

const USDC_TOKEN: &str = "0x3600000000000000000000000000000000000000";
const EURC_TOKEN: &str = "0x89b50855aa3be2f677cd6303cec089b5f319d72a";
const EEEE_ADDR: &str = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

const ADD_AMOUNTS: [u64; 2] = [10000, 0];
const MIN_MINT: u64 = 0;
const STAKE_AMOUNT: u64 = 1_000_000_000_000_000;

struct SwapDef {
    label: &'static str,
    route_a: &'static str,
    route_b: &'static str,
    route_c: &'static str,
    row: [u64; 4],
    amount_in: u128,
    native: bool,
}

const SWAP_DEFS: &[SwapDef] = &[
    SwapDef {
        label: "USDC to WUSDC",
        route_a: EEEE_ADDR,
        route_b: "0x911b4000d3422f482f4062a913885f7b035382df",
        route_c: "0x911b4000d3422f482f4062a913885f7b035382df",
        row: [0, 0, 8, 0],
        amount_in: 100_000_000_000_000_000,
        native: true,
    },
    SwapDef {
        label: "WUSDC to WBTC",
        route_a: "0x3600000000000000000000000000000000000000",
        route_b: "0x0ead9f3686adec44bc6976595fcb6427a90d5130",
        route_c: "0xc0cd72fefe31ea209b259f157a402861b7d425d4",
        row: [1, 0, 1, 20],
        amount_in: 100_000,
        native: false,
    },
    SwapDef {
        label: "WUSDC to ART",
        route_a: "0x911b4000d3422f482f4062a913885f7b035382df",
        route_b: "0x0df65440f7b1e0467b4399f4de2edeb96bafec2d",
        route_c: "0x912a5f3b5ebc55e4aa0424e4fd33507554557851",
        row: [0, 1, 1, 20],
        amount_in: 10_000_000_000_000_000,
        native: false,
    },
];

fn swap_key_to_index(key: &str) -> Option<usize> {
    match key {
        "usdc_to_wusdc" => Some(0),
        "wusdc_to_wbtc" => Some(1),
        "wusdc_to_art"  => Some(2),
        _ => None,
    }
}

async fn quote_out(client: &Client, data: Bytes, value: U256) -> U256 {
    let dex: Address = DEX_CONTRACT.parse().unwrap();
    let tx = ethers::types::TransactionRequest::new()
        .from(client.address())
        .to(dex)
        .data(data)
        .value(value);
    match client.call(&tx.into(), None).await {
        Ok(ret) if ret.len() >= 32 => U256::from_big_endian(&ret[..32]),
        _ => U256::zero(),
    }
}

fn apply_slippage(amount_out: U256, slippage_bps: u32) -> U256 {
    if amount_out.is_zero() { return U256::zero(); }
    let bps = slippage_bps.min(5000) as u64;
    amount_out * U256::from(10000 - bps) / U256::from(10000u64)
}

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client.get_transaction_count(client.address(), None).await.unwrap_or(fallback)
}

pub async fn run(client: &Client, mut nonce: U256, cfg: &CurvedexConfig) -> Result<U256> {
    ly("Curve Dex category execution started");

    let dex = Address::from_str(DEX_CONTRACT).unwrap();
    let pool_usdc_eurc = Address::from_str(POOL_USDC_EURC).unwrap();
    let stake_contract = Address::from_str(STAKE_CONTRACT).unwrap();
    let usdc = Address::from_str(USDC_TOKEN).unwrap();
    let eurc = Address::from_str(EURC_TOKEN).unwrap();
    let eeee: Address = EEEE_ADDR.parse().unwrap();

    let slippage_bps = cfg.slippage_bps.unwrap_or(50);

    if let Some(swaps_cfg) = &cfg.swaps {
        let keys_and_counts = [
            ("usdc_to_wusdc", swaps_cfg.usdc_to_wusdc.unwrap_or(0)),
            ("wusdc_to_wbtc", swaps_cfg.wusdc_to_wbtc.unwrap_or(0)),
            ("wusdc_to_art",  swaps_cfg.wusdc_to_art.unwrap_or(0)),
        ];

        for (key, count) in &keys_and_counts {
            if *count == 0 { continue; }
            let idx = match swap_key_to_index(key) {
                Some(i) => i,
                None => continue,
            };
            let def = &SWAP_DEFS[idx];
            let route_a: Address = def.route_a.parse().unwrap();
            let route_b: Address = def.route_b.parse().unwrap();
            let route_c: Address = def.route_c.parse().unwrap();
            let amount_in = U256::from(def.amount_in);
            let value = if def.native { amount_in } else { U256::zero() };

            if !def.native && route_a != eeee {
                match silent_approve(client, nonce, route_a, dex, U256::MAX).await {
                    Ok(n) => nonce = n,
                    Err(e) => {
                        lr(&format!("Curve Dex approve for {} failed: {}", def.label, e));
                        continue;
                    }
                }
            }

            for _ in 0..*count {
                let data_quote = calldata::build_exchange(route_a, route_b, route_c, def.row, amount_in, U256::zero());
                let quoted = quote_out(client, data_quote, value).await;
                if quoted.is_zero() {
                    ly(&format!("Curve Dex swap {} skipped because pool returned zero quote", def.label));
                    break;
                }
                let min_out = apply_slippage(quoted, slippage_bps);
                let data = calldata::build_exchange(route_a, route_b, route_c, def.row, amount_in, min_out);

                let ok = preflight_check(client, dex, data.clone(), value).await;
                if !ok {
                    ly(&format!("Curve Dex swap {} skipped because simulation did not pass", def.label));
                    break;
                }

                match execute_tx(client, nonce, dex, data, value, 560000, &format!("Curve Dex Swap {}", def.label)).await {
                    Ok(n) => nonce = n,
                    Err(e) => {
                        lr(&format!("Curve Dex swap {} failed: {}", def.label, e));
                        break;
                    }
                }
            }
        }
    }

    nonce = refresh_nonce(client, nonce).await;

    if let Some(addlp_cfg) = &cfg.add_lp {
        let usdc_eurc = addlp_cfg.usdc_eurc.unwrap_or(0);
        for _ in 0..usdc_eurc {
            match silent_approve(client, nonce, usdc, pool_usdc_eurc, U256::MAX).await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Curve Dex Add LP approve USDC failed: {}", e));
                    continue;
                }
            }

            match silent_approve(client, nonce, eurc, pool_usdc_eurc, U256::MAX).await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Curve Dex Add LP approve EURC failed: {}", e));
                    continue;
                }
            }

            match execute_tx(
                client, nonce, pool_usdc_eurc,
                calldata::build_add_liquidity(&ADD_AMOUNTS, MIN_MINT),
                U256::zero(), 620000, "Curve Dex Add LP USDC/EURC",
            ).await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Curve Dex Add LP USDC/EURC failed: {}", e)),
            }
        }
    }

    nonce = refresh_nonce(client, nonce).await;

    if let Some(stake_cfg) = &cfg.stake_deposit {
        let usdc_eurc = stake_cfg.usdc_eurc.unwrap_or(0);
        for _ in 0..usdc_eurc {
            match silent_approve(client, nonce, pool_usdc_eurc, stake_contract, U256::MAX).await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Curve Dex Stake approve LP failed: {}", e));
                    continue;
                }
            }

            match execute_tx(
                client, nonce, stake_contract,
                calldata::build_stake(U256::from(STAKE_AMOUNT)),
                U256::zero(), 260000, "Curve Dex Stake Deposit",
            ).await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Curve Dex Stake Deposit failed: {}", e)),
            }
        }
    }

    Ok(nonce)
}