use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub watchoor: Option<SimpleCategory>,
    pub super_bridge: Option<SimpleCategory>,
    pub zkcodex: Option<ZkcodexConfig>,
    pub onchaingm: Option<OnchainGmConfig>,
    pub swaparc: Option<SwaparcConfig>,
    pub loop_cycle: Option<LoopConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZkcodexConfig {
    pub enabled: bool,
    pub say_gm: Option<ZkcodexSayGmConfig>,
    pub deploy_contract: Option<SimpleCategory>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZkcodexSayGmConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimpleCategory {
    pub enabled: bool,
    pub times: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OnchainGmConfig {
    pub enabled: bool,
    pub gm_onchain: Option<SimpleCategory>,
    pub mint_badge: Option<SimpleCategory>,
    pub deploy: Option<SimpleCategory>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SwaparcConfig {
    pub enabled: bool,
    pub swaps: Option<SwaparcSwapsConfig>,
    pub add_lp: Option<SwaparcAddLpConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SwaparcSwapsConfig {
    pub usdc_to_swprc: Option<u32>,
    pub eurc_to_swprc: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SwaparcAddLpConfig {
    pub usdc_eurc: Option<u32>,
    pub usdc_swprc: Option<u32>,
    pub eurc_swprc: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoopConfig {
    pub enabled: bool,
    pub sleep_seconds: u64,
}

impl Config {
    pub fn watchoor_enabled(&self) -> bool {
        self.watchoor.as_ref().map(|c| c.enabled && c.times > 0).unwrap_or(false)
    }

    pub fn watchoor_times(&self) -> u32 {
        self.watchoor.as_ref().map(|c| c.times).unwrap_or(0)
    }

    pub fn super_bridge_enabled(&self) -> bool {
        self.super_bridge.as_ref().map(|c| c.enabled && c.times > 0).unwrap_or(false)
    }

    pub fn super_bridge_times(&self) -> u32 {
        self.super_bridge.as_ref().map(|c| c.times).unwrap_or(0)
    }

    pub fn zkcodex_enabled(&self) -> bool {
        self.zkcodex.as_ref().map(|c| {
            if !c.enabled { return false; }
            let say = c.say_gm.as_ref().map(|x| x.enabled).unwrap_or(false);
            let deploy = c.deploy_contract.as_ref().map(|x| x.times > 0).unwrap_or(false);
            say || deploy
        }).unwrap_or(false)
    }

    pub fn zkcodex_config(&self) -> Option<&ZkcodexConfig> {
        self.zkcodex.as_ref()
    }

    pub fn onchaingm_enabled(&self) -> bool {
        self.onchaingm.as_ref().map(|c| {
            if !c.enabled { return false; }
            let gm = c.gm_onchain.as_ref().map(|x| x.enabled && x.times > 0).unwrap_or(false);
            let badge = c.mint_badge.as_ref().map(|x| x.enabled && x.times > 0).unwrap_or(false);
            let deploy = c.deploy.as_ref().map(|x| x.enabled && x.times > 0).unwrap_or(false);
            gm || badge || deploy
        }).unwrap_or(false)
    }

    pub fn onchaingm_config(&self) -> Option<&OnchainGmConfig> {
        self.onchaingm.as_ref()
    }

    pub fn swaparc_enabled(&self) -> bool {
        self.swaparc.as_ref().map(|c| {
            if !c.enabled { return false; }
            let usdc_swprc = c.swaps.as_ref().and_then(|s| s.usdc_to_swprc).unwrap_or(0) > 0;
            let eurc_swprc = c.swaps.as_ref().and_then(|s| s.eurc_to_swprc).unwrap_or(0) > 0;
            let lp_usdc_eurc = c.add_lp.as_ref().and_then(|a| a.usdc_eurc).unwrap_or(0) > 0;
            let lp_usdc_swprc = c.add_lp.as_ref().and_then(|a| a.usdc_swprc).unwrap_or(0) > 0;
            let lp_eurc_swprc = c.add_lp.as_ref().and_then(|a| a.eurc_swprc).unwrap_or(0) > 0;
            usdc_swprc || eurc_swprc || lp_usdc_eurc || lp_usdc_swprc || lp_eurc_swprc
        }).unwrap_or(false)
    }

    pub fn swaparc_config(&self) -> Option<&SwaparcConfig> {
        self.swaparc.as_ref()
    }

    pub fn loop_enabled(&self) -> bool {
        self.loop_cycle.as_ref().map(|c| c.enabled).unwrap_or(false)
    }

    pub fn loop_sleep_seconds(&self) -> u64 {
        self.loop_cycle.as_ref().map(|c| c.sleep_seconds).unwrap_or(7200)
    }
}

pub fn load_config() -> Result<Config> {
    let cfg_path = config_path();

    if !cfg_path.exists() {
        let sample = serde_json::json!({
            "watchoor": { "enabled": true, "times": 1 },
            "super_bridge": { "enabled": true, "times": 1 },
            "zkcodex": {
                "enabled": true,
                "say_gm": { "enabled": true },
                "deploy_contract": { "enabled": true, "times": 1 }
            },
            "onchaingm": {
                "enabled": true,
                "gm_onchain": { "enabled": true, "times": 1 },
                "mint_badge": { "enabled": true, "times": 1 },
                "deploy": { "enabled": true, "times": 1 }
            },
            "swaparc": {
                "enabled": true,
                "swaps": { "usdc_to_swprc": 1, "eurc_to_swprc": 1 },
                "add_lp": { "usdc_eurc": 1, "usdc_swprc": 1, "eurc_swprc": 1 }
            },
            "loop_cycle": { "enabled": false, "sleep_seconds": 7200 }
        });
        fs::write(&cfg_path, serde_json::to_string_pretty(&sample)?)
            .context("Config file could not be written")?;
        anyhow::bail!("Config file has been created, please edit and rerun");
    }

    let raw = fs::read_to_string(&cfg_path).context("Config file could not be read")?;
    let cfg: Config = serde_json::from_str(&raw).context("Config file format is not valid")?;
    Ok(cfg)
}

fn config_path() -> PathBuf {
    let mut p = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    p.pop();
    p.push("config.json");
    if !p.exists() {
        let local = PathBuf::from("config.json");
        if local.exists() {
            return local;
        }
    }
    p
}