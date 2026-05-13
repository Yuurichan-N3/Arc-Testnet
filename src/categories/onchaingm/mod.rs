pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::config::OnchainGmConfig;
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, preflight_check, Client};

const GM_CONTRACT: &str = "0x363cc75a89ae5673b427a1fa98afc48ffde7ba43";
const BADGE_CONTRACT: &str = "0xc5c68144edac2e1872f744c2b07acc5a78f6e63e";
const DEPLOY_CONTRACT: &str = "0xa3d9fbd0edb10327ecb73d2c72622e505df468a2";

const GM_VALUE_WEI: u64 = 0x58d15e176280000;
const BADGE_VALUE_WEI: u64 = 0xde0b6b3a7640000;
const DEPLOY_VALUE_WEI: u64 = 0xde0b6b3a7640000;

pub async fn run(client: &Client, mut nonce: U256, cfg: &OnchainGmConfig) -> Result<U256> {
    ly("OnchainGM category execution started");

    let gm_contract = Address::from_str(GM_CONTRACT).unwrap();
    let badge_contract = Address::from_str(BADGE_CONTRACT).unwrap();
    let deploy_contract = Address::from_str(DEPLOY_CONTRACT).unwrap();

    let mut gm_already_done = false;

    if let Some(gm_cfg) = &cfg.gm_onchain {
        if gm_cfg.enabled && gm_cfg.times > 0 {
            for _ in 0..gm_cfg.times {
                let data = calldata::build_gm();
                let ok = preflight_check(client, gm_contract, data.clone(), U256::from(GM_VALUE_WEI)).await;
                if !ok {
                    ly("GM Onchain appears already completed and skipped");
                    gm_already_done = true;
                    break;
                }
                match execute_tx(
                    client, nonce, gm_contract,
                    data,
                    U256::from(GM_VALUE_WEI), 260000, "GM Onchain",
                ).await {
                    Ok(next_nonce) => nonce = next_nonce,
                    Err(_) => {
                        ly("GM Onchain appears already completed and skipped");
                        gm_already_done = true;
                        break;
                    }
                }
            }
        }
    }

    if gm_already_done {
        ly("Mint Badge and Deploy skipped because GM already completed");
        return Ok(nonce);
    }

    if let Some(badge_cfg) = &cfg.mint_badge {
        if badge_cfg.enabled && badge_cfg.times > 0 {
            for _ in 0..badge_cfg.times {
                nonce = execute_tx(
                    client, nonce, badge_contract,
                    calldata::build_mint_badge(),
                    U256::from(BADGE_VALUE_WEI), 260000, "OnchainGM Mint Badge",
                )
                .await
                .map_err(|e| { lr(&format!("OnchainGM Mint Badge failed: {}", e)); e })?;
            }
        }
    }

    if let Some(deploy_cfg) = &cfg.deploy {
        if deploy_cfg.enabled && deploy_cfg.times > 0 {
            for _ in 0..deploy_cfg.times {
                nonce = execute_tx(
                    client, nonce, deploy_contract,
                    calldata::build_deploy(),
                    U256::from(DEPLOY_VALUE_WEI), 180000, "OnchainGM Deploy",
                )
                .await
                .map_err(|e| { lr(&format!("OnchainGM Deploy failed: {}", e)); e })?;
            }
        }
    }

    Ok(nonce)
}