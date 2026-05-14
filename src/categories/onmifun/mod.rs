pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::OnmifunConfig;
use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, silent_approve, Client};

const ROUTER: &str = "0xdA85977039e0A69AFcC3282Ec980cFEdd664a538";
const WUSDC: &str = "0x911b4000D3422F482F4062a913885f7b035382Df";
const CHNOS: &str = "0xC7E6669e7F83Ae7f38fcB459C192fF7e843543Dd";
const MOG: &str = "0x3BAD69807A01367147416D3e36c03275348E95d5";

const SWAP_ETH_VALUE: u64 = 1_000_000_000_000_000;
const ADD_LP_ETH_VALUE: u64 = 100_000_000_000_000;
const ADD_LP_TOKEN_DESIRED: u64 = 100_000_000_000_000_000;
const ADD_LP_TOKEN_MIN: u64 = 0;
const ADD_LP_ETH_MIN: u64 = 0;

fn deadline() -> U256 {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    U256::from(ts + 600)
}

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

pub async fn run(client: &Client, mut nonce: U256, cfg: &OnmifunConfig) -> Result<U256> {
    ly("Onmifun category execution started");

    let router = Address::from_str(ROUTER).unwrap();
    let wusdc = Address::from_str(WUSDC).unwrap();
    let chnos = Address::from_str(CHNOS).unwrap();
    let mog = Address::from_str(MOG).unwrap();
    let to = client.address();

    if let Some(swap_cfg) = &cfg.swap {
        if swap_cfg.enabled {
            let swap_value = U256::from(SWAP_ETH_VALUE);
            let dl = deadline();

            let data_chnos = calldata::build_swap_exact_eth_for_tokens(
                U256::zero(),
                vec![wusdc, chnos],
                to,
                dl,
            );
            match execute_tx(client, nonce, router, data_chnos, swap_value, 400000, "Onmifun Swap ETH to CHNOS").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Onmifun Swap ETH to CHNOS failed: {}", e)),
            }

            let dl = deadline();
            let data_mog = calldata::build_swap_exact_eth_for_tokens(
                U256::zero(),
                vec![wusdc, mog],
                to,
                dl,
            );
            match execute_tx(client, nonce, router, data_mog, swap_value, 400000, "Onmifun Swap ETH to MOG").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Onmifun Swap ETH to MOG failed: {}", e)),
            }

            lg("Onmifun swap execution completed");
        }
    }

    nonce = refresh_nonce(client, nonce).await;

    if let Some(lp_cfg) = &cfg.add_lp {
        if lp_cfg.enabled {
            let lp_value = U256::from(ADD_LP_ETH_VALUE);
            let token_desired = U256::from(ADD_LP_TOKEN_DESIRED);
            let token_min = U256::from(ADD_LP_TOKEN_MIN);
            let eth_min = U256::from(ADD_LP_ETH_MIN);

            match silent_approve(client, nonce, chnos, router, U256::MAX).await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Onmifun Add LP CHNOS approve failed: {}", e));
                }
            }

            let dl = deadline();
            let data_chnos_lp = calldata::build_add_liquidity_eth(
                chnos,
                token_desired,
                token_min,
                eth_min,
                to,
                dl,
            );
            match execute_tx(client, nonce, router, data_chnos_lp, lp_value, 500000, "Onmifun Add LP ETH/CHNOS").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Onmifun Add LP ETH/CHNOS failed: {}", e)),
            }

            match silent_approve(client, nonce, mog, router, U256::MAX).await {
                Ok(n) => nonce = n,
                Err(e) => {
                    lr(&format!("Onmifun Add LP MOG approve failed: {}", e));
                }
            }

            let dl = deadline();
            let data_mog_lp = calldata::build_add_liquidity_eth(
                mog,
                token_desired,
                token_min,
                eth_min,
                to,
                dl,
            );
            match execute_tx(client, nonce, router, data_mog_lp, lp_value, 500000, "Onmifun Add LP ETH/MOG").await {
                Ok(n) => nonce = n,
                Err(e) => lr(&format!("Onmifun Add LP ETH/MOG failed: {}", e)),
            }

            lg("Onmifun add liquidity execution completed");
        }
    }

    Ok(nonce)
}