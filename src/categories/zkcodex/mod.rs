pub mod calldata;

use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::config::ZkcodexConfig;
use crate::logger::{lr, ly};
use crate::rpc::{execute_tx, preflight_check, Client};

const SAY_GM_CONTRACT: &str = "0x1290b4f2a419a316467b580a088453a233e9adcc";
const DEPLOY_CONTRACT: &str = "0xecf3365559ffe5fdbe1953df0a01244e234e4453";

const SAY_GM_VALUE_WEI: u64 = 0;
const DEPLOY_VALUE_WEI: u64 = 0x16345785d8a0000;

const SAY_GM_MESSAGE: &str = "GM!";

pub async fn run(client: &Client, mut nonce: U256, cfg: &ZkcodexConfig) -> Result<U256> {
    ly("ZK Codex category execution started");

    let say_gm_contract = Address::from_str(SAY_GM_CONTRACT).unwrap();
    let deploy_contract = Address::from_str(DEPLOY_CONTRACT).unwrap();

    if let Some(say_cfg) = &cfg.say_gm {
        if say_cfg.enabled {
            let data = calldata::build_say_gm(SAY_GM_MESSAGE);
            let ok = preflight_check(
                client,
                say_gm_contract,
                data.clone(),
                U256::from(SAY_GM_VALUE_WEI),
            )
            .await;

            if !ok {
                ly("ZK Codex Say GM appears already completed today and skipped");
            } else {
                match execute_tx(
                    client,
                    nonce,
                    say_gm_contract,
                    data,
                    U256::from(SAY_GM_VALUE_WEI),
                    220000,
                    "ZK Codex Say GM",
                )
                .await
                {
                    Ok(next_nonce) => nonce = next_nonce,
                    Err(e) => lr(&format!("ZK Codex Say GM failed: {}", e)),
                }
            }
        }
    }

    if let Some(deploy_cfg) = &cfg.deploy_contract {
        if deploy_cfg.times > 0 {
            for _ in 0..deploy_cfg.times {
                nonce = execute_tx(
                    client,
                    nonce,
                    deploy_contract,
                    calldata::build_deploy_contract(),
                    U256::from(DEPLOY_VALUE_WEI),
                    520000,
                    "ZK Codex Deploy Contract",
                )
                .await
                .map_err(|e| {
                    lr(&format!("ZK Codex Deploy Contract failed: {}", e));
                    e
                })?;
            }
        }
    }

    Ok(nonce)
}
