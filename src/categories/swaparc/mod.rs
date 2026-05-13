pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::config::SwaparcConfig;
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, silent_approve, Client};

const SWAP_CONTRACT: &str = "0x2f4490e7c6f3dac23ffee6e71bfcb5d1ccd7d4ec";
const SWPRC_TOKEN: &str = "0xbe7477bf91526fc9988c8f33e91b6db687119d45";
const USDC_TOKEN: &str = "0x3600000000000000000000000000000000000000";
const EURC_TOKEN: &str = "0x89b50855aa3be2f677cd6303cec089b5f319d72a";

const POOL_USDC_EURC: &str = "0xd22e4fb80e21e8d2c91131ec2d6b0c000491934b";
const POOL_USDC_SWPRC: &str = "0x613bc8a188a571e7ffe3f884fabab0f43abb8282";
const POOL_EURC_SWPRC: &str = "0x9463de67e73b42b2ce5e45cab7e32184b9c24939";

const DX: u64 = 100000;
const ADD_AMOUNT: u64 = 100000;

pub async fn run(client: &Client, mut nonce: U256, cfg: &SwaparcConfig) -> Result<U256> {
    ly("SwapArc category execution started");

    let swap_contract = Address::from_str(SWAP_CONTRACT).unwrap();
    let usdc = Address::from_str(USDC_TOKEN).unwrap();
    let eurc = Address::from_str(EURC_TOKEN).unwrap();
    let swprc = Address::from_str(SWPRC_TOKEN).unwrap();

    // --- Swaps ---
    if let Some(swaps) = &cfg.swaps {
        let usdc_to_swprc = swaps.usdc_to_swprc.unwrap_or(0);
        for _ in 0..usdc_to_swprc {
            nonce = silent_approve(client, nonce, usdc, swap_contract, U256::from(DX))
                .await
                .map_err(|e| { lr(&format!("SwapArc approve USDC failed: {}", e)); e })?;

            nonce = execute_tx(
                client, nonce, swap_contract,
                calldata::build_swap(0, 2, DX),
                U256::zero(), 300000, "SwapArc Swap USDC to SWPRC",
            )
            .await
            .map_err(|e| { lr(&format!("SwapArc Swap USDC to SWPRC failed: {}", e)); e })?;
        }

        let eurc_to_swprc = swaps.eurc_to_swprc.unwrap_or(0);
        for _ in 0..eurc_to_swprc {
            nonce = silent_approve(client, nonce, eurc, swap_contract, U256::from(DX))
                .await
                .map_err(|e| { lr(&format!("SwapArc approve EURC failed: {}", e)); e })?;

            nonce = execute_tx(
                client, nonce, swap_contract,
                calldata::build_swap(1, 2, DX),
                U256::zero(), 300000, "SwapArc Swap EURC to SWPRC",
            )
            .await
            .map_err(|e| { lr(&format!("SwapArc Swap EURC to SWPRC failed: {}", e)); e })?;
        }
    }

    // --- Add LP ---
    if let Some(add_lp) = &cfg.add_lp {
        let pool_usdc_eurc = Address::from_str(POOL_USDC_EURC).unwrap();
        let pool_usdc_swprc = Address::from_str(POOL_USDC_SWPRC).unwrap();
        let pool_eurc_swprc = Address::from_str(POOL_EURC_SWPRC).unwrap();

        let usdc_eurc = add_lp.usdc_eurc.unwrap_or(0);
        for _ in 0..usdc_eurc {
            nonce = silent_approve(client, nonce, usdc, pool_usdc_eurc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP USDC/EURC approve USDC failed: {}", e)); e })?;

            nonce = silent_approve(client, nonce, eurc, pool_usdc_eurc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP USDC/EURC approve EURC failed: {}", e)); e })?;

            nonce = execute_tx(
                client, nonce, pool_usdc_eurc,
                calldata::build_add_liquidity(ADD_AMOUNT, 0),
                U256::zero(), 520000, "SwapArc Add LP USDC/EURC",
            )
            .await
            .map_err(|e| { lr(&format!("SwapArc Add LP USDC/EURC failed: {}", e)); e })?;
        }

        let usdc_swprc = add_lp.usdc_swprc.unwrap_or(0);
        for _ in 0..usdc_swprc {
            nonce = silent_approve(client, nonce, usdc, pool_usdc_swprc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP USDC/SWPRC approve USDC failed: {}", e)); e })?;

            nonce = silent_approve(client, nonce, swprc, pool_usdc_swprc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP USDC/SWPRC approve SWPRC failed: {}", e)); e })?;

            nonce = execute_tx(
                client, nonce, pool_usdc_swprc,
                calldata::build_add_liquidity(ADD_AMOUNT, 0),
                U256::zero(), 520000, "SwapArc Add LP USDC/SWPRC",
            )
            .await
            .map_err(|e| { lr(&format!("SwapArc Add LP USDC/SWPRC failed: {}", e)); e })?;
        }

        let eurc_swprc = add_lp.eurc_swprc.unwrap_or(0);
        for _ in 0..eurc_swprc {
            nonce = silent_approve(client, nonce, eurc, pool_eurc_swprc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP EURC/SWPRC approve EURC failed: {}", e)); e })?;

            nonce = silent_approve(client, nonce, swprc, pool_eurc_swprc, U256::from(ADD_AMOUNT))
                .await
                .map_err(|e| { lr(&format!("SwapArc Add LP EURC/SWPRC approve SWPRC failed: {}", e)); e })?;

            nonce = execute_tx(
                client, nonce, pool_eurc_swprc,
                calldata::build_add_liquidity(ADD_AMOUNT, 0),
                U256::zero(), 520000, "SwapArc Add LP EURC/SWPRC",
            )
            .await
            .map_err(|e| { lr(&format!("SwapArc Add LP EURC/SWPRC failed: {}", e)); e })?;
        }
    }

    Ok(nonce)
}