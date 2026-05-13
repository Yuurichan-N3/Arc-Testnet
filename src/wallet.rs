use anyhow::{Context, Result};
use ethers::signers::{LocalWallet, Signer};
use regex::Regex;
use std::path::PathBuf;

pub struct WalletEntry {
    pub idx: u32,
    pub wallet: LocalWallet,
}

pub fn load_wallets() -> Result<Vec<WalletEntry>> {
    let env_path = env_path();
    if env_path.exists() {
        dotenvy::from_path(&env_path).ok();
    } else {
        dotenvy::dotenv().ok();
    }

    let re = Regex::new(r"^PRIVATEKEY_(\d+)$").unwrap();
    let mut items: Vec<(u32, String)> = std::env::vars()
        .filter_map(|(k, v)| {
            let caps = re.captures(&k)?;
            let idx: u32 = caps[1].parse().ok()?;
            let pk = normalize_pk(&v).ok()?;
            Some((idx, pk))
        })
        .collect();

    if items.is_empty() {
        anyhow::bail!("No private keys found, use PRIVATEKEY_1 format inside the env file");
    }

    items.sort_by_key(|(i, _)| *i);

    let wallets = items
        .into_iter()
        .map(|(idx, pk)| {
            let wallet: LocalWallet = pk
                .parse()
                .with_context(|| format!("Private key {} could not be parsed", idx))?;
            Ok(WalletEntry { idx, wallet })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(wallets)
}

fn normalize_pk(pk: &str) -> Result<String> {
    let pk = pk.trim().trim_matches('"').trim_matches('\'');
    let pk = if let Some(stripped) = pk.strip_prefix("0x") {
        stripped
    } else {
        pk
    };
    let re = Regex::new(r"^[0-9a-fA-F]{64}$").unwrap();
    if !re.is_match(pk) {
        anyhow::bail!("Private key format is not valid");
    }
    Ok(format!("0x{}", pk.to_lowercase()))
}

fn env_path() -> PathBuf {
    let mut p = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    p.pop();
    p.push(".env");
    if !p.exists() {
        let local = PathBuf::from(".env");
        if local.exists() {
            return local;
        }
    }
    p
}
