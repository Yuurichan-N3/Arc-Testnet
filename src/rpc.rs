use anyhow::{Context, Result};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes, Eip1559TransactionRequest,
        TransactionRequest, U256, H256,
    },
};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::constants::*;
use crate::logger::{lg, lr, ly};
use crate::proxy::ProxyPool;

pub type Client = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub async fn connect_with_retry(proxy_pool: &ProxyPool, wallet: LocalWallet) -> Result<Client> {
    let mut delay = 2u64;
    let mut last_err = String::new();

    for attempt in 1..=6 {
        let provider_result = build_provider(proxy_pool);
        match provider_result {
            Ok(provider) => {
                let chain_id = CHAIN_ID;
                let wallet = wallet.clone().with_chain_id(chain_id);
                let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
                match provider.get_block_number().await {
                    Ok(_) => {
                        lg(&format!(
                            "RPC connection established via {}",
                            proxy_pool.current_label()
                        ));
                        return Ok(client);
                    }
                    Err(e) => {
                        last_err = crate::logger::sanitize(&e.to_string());
                    }
                }
            }
            Err(e) => {
                last_err = crate::logger::sanitize(&e.to_string());
            }
        }

        proxy_pool.rotate();
        if attempt < 6 {
            ly(&format!(
                "RPC connection attempt {} did not succeed retrying via {}",
                attempt,
                proxy_pool.current_label()
            ));
            tokio::time::sleep(Duration::from_secs(delay)).await;
            delay = (delay * 2).min(10);
        }
    }

    anyhow::bail!("RPC connection could not be established: {}", last_err)
}

fn build_provider(proxy_pool: &ProxyPool) -> Result<Provider<Http>> {
    if let Some(proxy_url) = proxy_pool.current_url() {
        let proxy = reqwest::Proxy::all(&proxy_url)
            .context("Proxy URL could not be parsed")?;
        let http_client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(30))
            .build()
            .context("HTTP client with proxy could not be built")?;
        let provider = Provider::new(Http::new_with_client(
            url::Url::parse(RPC_URL).unwrap(),
            http_client,
        ));
        return Ok(provider);
    }
    let provider = Provider::<Http>::try_from(RPC_URL).context("RPC URL could not be parsed")?;
    Ok(provider)
}

pub struct FeeParams {
    pub eip1559: bool,
    pub max_fee: U256,
    pub priority_fee: U256,
    pub gas_price: U256,
}

pub async fn get_fee_params(client: &Client) -> FeeParams {
    if let Ok(Some(block)) = client.get_block(ethers::types::BlockNumber::Pending).await {
        if let Some(base_fee) = block.base_fee_per_gas {
            let priority = U256::from(2_000_000_000u64);
            let max_fee = base_fee * 2 + priority;
            return FeeParams {
                eip1559: true,
                max_fee,
                priority_fee: priority,
                gas_price: U256::zero(),
            };
        }
    }
    let gas_price = client.get_gas_price().await.unwrap_or(U256::from(1_000_000_000u64));
    FeeParams {
        eip1559: false,
        max_fee: U256::zero(),
        priority_fee: U256::zero(),
        gas_price: gas_price * 11 / 10,
    }
}

pub async fn estimate_gas(
    client: &Client,
    from: Address,
    to: Address,
    data: Bytes,
    value: U256,
    fallback: u64,
) -> u64 {
    let tx = TransactionRequest::new()
        .from(from)
        .to(to)
        .data(data)
        .value(value);
    let typed: TypedTransaction = tx.into();
    match client.estimate_gas(&typed, None).await {
        Ok(g) => (g * U256::from(12) / U256::from(10) + U256::from(5000)).as_u64(),
        Err(_) => fallback,
    }
}

pub async fn wait_receipt(client: &Client, tx_hash: H256) -> Result<ethers::types::TransactionReceipt> {
    let start = Instant::now();
    let timeout = Duration::from_secs(RECEIPT_TIMEOUT_SEC);
    let poll = Duration::from_secs(RECEIPT_POLL_SEC);

    loop {
        match client.get_transaction_receipt(tx_hash).await {
            Ok(Some(rcpt)) => return Ok(rcpt),
            _ => {}
        }
        if start.elapsed() > timeout {
            anyhow::bail!("Receipt wait timed out");
        }
        tokio::time::sleep(poll).await;
    }
}

pub async fn execute_tx(
    client: &Client,
    nonce: U256,
    to: Address,
    data: Bytes,
    value: U256,
    gas_fallback: u64,
    title: &str,
) -> Result<U256> {
    ly(&format!("{} transaction is now starting", title));

    let from = client.address();
    let fee = get_fee_params(client).await;
    let gas = estimate_gas(client, from, to, data.clone(), value, gas_fallback).await;

    let tx: TypedTransaction = if fee.eip1559 {
        Eip1559TransactionRequest::new()
            .from(from)
            .to(to)
            .nonce(nonce)
            .data(data)
            .value(value)
            .gas(gas)
            .max_fee_per_gas(fee.max_fee)
            .max_priority_fee_per_gas(fee.priority_fee)
            .chain_id(CHAIN_ID)
            .into()
    } else {
        TransactionRequest::new()
            .from(from)
            .to(to)
            .nonce(nonce)
            .data(data)
            .value(value)
            .gas(gas)
            .gas_price(fee.gas_price)
            .chain_id(CHAIN_ID)
            .into()
    };

    let pending = client
        .send_transaction(tx, None)
        .await
        .context("Transaction broadcast failed")?;

    let tx_hash = pending.tx_hash();
    lg("Transaction broadcast completed successfully");
    lg(&format!("Transaction hash {:x}", tx_hash));

    let rcpt = wait_receipt(client, tx_hash).await?;
    let status = rcpt.status.map(|s| s.as_u64()).unwrap_or(0);

    if status == 1 {
        lg("Receipt confirmed with successful execution");
    } else {
        lr("Receipt confirmed with failed execution");
        anyhow::bail!("Transaction execution failed");
    }

    Ok(nonce + 1)
}

pub async fn silent_approve(
    client: &Client,
    nonce: U256,
    token: Address,
    spender: Address,
    amount: U256,
) -> Result<U256> {
    let data = crate::calldata::build_approve(spender, amount);
    execute_tx(client, nonce, token, data, U256::zero(), 120000, "Allowance update").await
}

pub async fn preflight_check(
    client: &Client,
    to: Address,
    data: Bytes,
    value: U256,
) -> bool {
    let from = client.address();
    let tx = TransactionRequest::new()
        .from(from)
        .to(to)
        .data(data)
        .value(value);
    let typed: TypedTransaction = tx.into();
    client.call(&typed, None).await.is_ok()
}