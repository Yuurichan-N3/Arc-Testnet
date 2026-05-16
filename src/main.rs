mod banner;
mod calldata;
mod categories;
mod config;
mod constants;
mod logger;
mod proxy;
mod rpc;
mod wallet;

use anyhow::Result;
use ethers::providers::Middleware;
use logger::{lg, lr, ly};
use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::select! {
        result = run() => {
            if let Err(e) = result {
                lr(&format!("Program ended due to unexpected error: {}", logger::sanitize(&e.to_string())));
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!();
            lr("Script stopped by user");
        }
    }
}

async fn run() -> Result<()> {
    banner::print_banner();

    let cfg = config::load_config().map_err(|e| {
        ly(&e.to_string());
        e
    })?;

    let proxy_pool = proxy::ProxyPool::load();
    let wallets = wallet::load_wallets().map_err(|e| {
        lr(&e.to_string());
        e
    })?;

    let mut cycle_no: u32 = 0;

    loop {
        cycle_no += 1;
        ly(&format!("Cycle {} execution started", cycle_no));

        for (i, entry) in wallets.iter().enumerate() {
            if i > 0 {
                println!();
            }

            let client = match rpc::connect_with_retry(&proxy_pool, entry.wallet.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    lr(&format!(
                        "Wallet {} RPC connection failed: {}",
                        entry.idx,
                        logger::sanitize(&e.to_string())
                    ));
                    continue;
                }
            };

            let addr = client.address();
            lg(&format!("Wallet {} processing started", entry.idx));
            lg(&format!("Wallet address {:?}", addr));

            let nonce = match client.get_transaction_count(addr, None).await {
                Ok(n) => n,
                Err(e) => {
                    lr(&format!(
                        "Wallet {} nonce fetch failed: {}",
                        entry.idx,
                        logger::sanitize(&e.to_string())
                    ));
                    continue;
                }
            };

            let mut nonce = nonce;
            let mut did_any = false;

            if cfg.watchoor_enabled() {
                if did_any { println!(); }
                match categories::watchoor::run(&client, nonce, cfg.watchoor_times()).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Watchoor category failed and continued"),
                }
                did_any = true;
            }

            if cfg.super_bridge_enabled() {
                if did_any { println!(); }
                match categories::super_bridge::run(&client, nonce, cfg.super_bridge_times()).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Super bridge category failed and continued"),
                }
                did_any = true;
            }

            if cfg.zkcodex_enabled() {
                if did_any { println!(); }
                if let Some(zcfg) = cfg.zkcodex_config() {
                    match categories::zkcodex::run(&client, nonce, zcfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("ZK Codex category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.onchaingm_enabled() {
                if did_any { println!(); }
                if let Some(ocfg) = cfg.onchaingm_config() {
                    match categories::onchaingm::run(&client, nonce, ocfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("OnchainGM category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.swaparc_enabled() {
                if did_any { println!(); }
                if let Some(scfg) = cfg.swaparc_config() {
                    match categories::swaparc::run(&client, nonce, scfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("SwapArc category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.axpha_enabled() {
                if did_any { println!(); }
                if let Some(acfg) = cfg.axpha_config() {
                    match categories::axpha::run(&client, nonce, acfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("Axpha category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.curvedex_enabled() {
                if did_any { println!(); }
                if let Some(ccfg) = cfg.curvedex_config() {
                    match categories::curvedex::run(&client, nonce, ccfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("Curve Dex category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.sweethaus_enabled() {
                if did_any { println!(); }
                if let Some(scfg) = cfg.sweethaus_config() {
                    match categories::sweethaus::run(&client, nonce, scfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("Sweet Haus category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.onmifun_enabled() {
                if did_any { println!(); }
                if let Some(ocfg) = cfg.onmifun_config() {
                    match categories::onmifun::run(&client, nonce, ocfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("Onmifun category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.flowthree_enabled() {
                if did_any { println!(); }
                match categories::flowthree::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Flow Three category failed and continued"),
                }
                did_any = true;
            }

            if cfg.omnihub_enabled() {
                if did_any { println!(); }
                match categories::omnihub::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Omnihub category failed and continued"),
                }
                did_any = true;
            }

            if cfg.flowonarc_enabled() {
                if did_any { println!(); }
                match categories::flowonarc::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Flow On Arc category failed and continued"),
                }
                did_any = true;
            }

            if cfg.prestodex_enabled() {
                if did_any { println!(); }
                if let Some(pcfg) = cfg.prestodex_config() {
                    match categories::prestodex::run(&client, nonce, pcfg).await {
                        Ok(n) => nonce = n,
                        Err(_) => lr("Presto Dex category failed and continued"),
                    }
                }
                did_any = true;
            }

            if cfg.painitiepay_enabled() {
                if did_any { println!(); }
                match categories::painitiepay::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Painitiepay category failed and continued"),
                }
                did_any = true;
            }

            if cfg.paytag_enabled() {
                if did_any { println!(); }
                match categories::paytag::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Paytag category failed and continued"),
                }
                did_any = true;
            }

            if cfg.arcfx_enabled() {
                if did_any { println!(); }
                match categories::arcfx::run(&client, nonce).await {
                    Ok(n) => nonce = n,
                    Err(_) => lr("Arc FX category failed and continued"),
                }
                did_any = true;
            }

            let _ = (nonce, did_any);
            lg(&format!("Wallet {} processing completed", entry.idx));
        }

        lg(&format!("Cycle {} execution completed", cycle_no));

        if !cfg.loop_enabled() {
            break;
        }

        sleep_countdown(cfg.loop_sleep_seconds()).await;
    }

    Ok(())
}

async fn sleep_countdown(seconds: u64) {
    let mut remaining = seconds;
    while remaining > 0 {
        let h = remaining / 3600;
        let m = (remaining % 3600) / 60;
        let s = remaining % 60;
        print!("\r\x1b[1;33mNext cycle in {:02}:{:02}:{:02}\x1b[0m", h, m, s);
        let _ = std::io::Write::flush(&mut std::io::stdout());
        tokio::time::sleep(Duration::from_secs(1)).await;
        remaining -= 1;
    }
    println!();
}