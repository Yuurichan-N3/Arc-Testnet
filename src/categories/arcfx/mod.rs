pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, Bytes, TransactionRequest, U256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const FAUCET: &str = "0x256B553b2Db34a0B10536cB4628610aFF4E1e7f6";
const ROUTER: &str = "0xce894c000F4003e3F45F9422b6E47EEcf1eAe4b0";
const CHECKIN: &str = "0x0d08131435cf890e8B4426EA3E0e3B4425c3b33e";
const STAKE: &str = "0x3554D4d10682fdc680A2cb64ADa35f8E7a297a32";
const USDC: &str = "0x3600000000000000000000000000000000000000";
const AUSDC: &str = "0xeD7cb772b49448027901546870425579596faaE1";
const AJPYC: &str = "0x7b765B44C9AF5EBb191296A05C8b9df5085f1f09";
const ATRYC: &str = "0x8DD16a98A3f5d767d5D08bEECbEa1Cd8CF2832ee";
const AGBPC: &str = "0x6374151C499DADc9A54650D25CdFF3B5688652Ba";
const AEURC: &str = "0x429a1D105558f4727453d2a17dF17ac9d5be1EA9";

const SWAP_AMOUNT_IN: u64 = 100_000_000_000_000_000;
const SWAP_AMOUNT_OUT_MIN: u64 = 1;
const LP_ATRYC_AJPYC_A0: u64 = 100_000_000_000_000_000;
const LP_ATRYC_AJPYC_A1: u64 = 348_170_000_000_000_000;
const LP_AGBPC_AEURC_A0: u64 = 100_000_000_000_000_000;
const LP_AGBPC_AEURC_A1: u64 = 8_640_000_000_000_000;
const STAKE_AMOUNT: u64 = 100_000;

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

async fn can_claim(client: &Client, contract: Address, data: Bytes) -> bool {
    let tx = TransactionRequest::new()
        .from(client.address())
        .to(contract)
        .data(data)
        .value(U256::zero());
    client.call(&tx.into(), None).await.is_ok()
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Arc FX category execution started");

    let faucet = Address::from_str(FAUCET).unwrap();
    let router = Address::from_str(ROUTER).unwrap();
    let checkin = Address::from_str(CHECKIN).unwrap();
    let stake = Address::from_str(STAKE).unwrap();
    let usdc = Address::from_str(USDC).unwrap();
    let ausdc = Address::from_str(AUSDC).unwrap();
    let ajpyc = Address::from_str(AJPYC).unwrap();
    let atryc = Address::from_str(ATRYC).unwrap();
    let agbpc = Address::from_str(AGBPC).unwrap();
    let aeurc = Address::from_str(AEURC).unwrap();
    let to = client.address();

    if can_claim(client, faucet, calldata::build_claim()).await {
        match execute_tx(client, nonce, faucet, calldata::build_claim(), U256::zero(), 400000, "Arc FX Faucet Claim").await {
            Ok(n) => { nonce = n; lg("Arc FX faucet claim completed"); }
            Err(e) => lr(&format!("Arc FX Faucet Claim failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    } else {
        ly("Arc FX faucet already claimed this period, skipping");
    }

    let approve_ajpyc = calldata::build_approve(router, U256::from(SWAP_AMOUNT_IN));
    match execute_tx(client, nonce, ajpyc, approve_ajpyc, U256::zero(), 100000, "Arc FX Approve aJPYC").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve aJPYC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client, nonce, router,
        calldata::build_swap_exact_tokens_for_tokens(
            U256::from(SWAP_AMOUNT_IN),
            U256::from(SWAP_AMOUNT_OUT_MIN),
            vec![ajpyc, ausdc],
            to,
            deadline(),
        ),
        U256::zero(), 300000, "Arc FX Swap aJPYC to aUSDC",
    ).await {
        Ok(n) => { nonce = n; lg("Arc FX swap aJPYC to aUSDC completed"); }
        Err(e) => lr(&format!("Arc FX Swap aJPYC to aUSDC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let approve_atryc = calldata::build_approve(router, U256::from(LP_ATRYC_AJPYC_A0));
    match execute_tx(client, nonce, atryc, approve_atryc, U256::zero(), 100000, "Arc FX Approve aTRYC").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve aTRYC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let approve_ajpyc_lp = calldata::build_approve(router, U256::from(LP_ATRYC_AJPYC_A1));
    match execute_tx(client, nonce, ajpyc, approve_ajpyc_lp, U256::zero(), 100000, "Arc FX Approve aJPYC for LP").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve aJPYC for LP failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client, nonce, router,
        calldata::build_add_liquidity(
            atryc, ajpyc,
            U256::from(LP_ATRYC_AJPYC_A0),
            U256::from(LP_ATRYC_AJPYC_A1),
            to,
        ),
        U256::zero(), 2000000, "Arc FX Add LP aTRYC/aJPYC",
    ).await {
        Ok(n) => { nonce = n; lg("Arc FX add LP aTRYC/aJPYC completed"); }
        Err(e) => lr(&format!("Arc FX Add LP aTRYC/aJPYC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let approve_agbpc = calldata::build_approve(router, U256::from(LP_AGBPC_AEURC_A0));
    match execute_tx(client, nonce, agbpc, approve_agbpc, U256::zero(), 100000, "Arc FX Approve aGBPC").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve aGBPC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    let approve_aeurc = calldata::build_approve(router, U256::from(LP_AGBPC_AEURC_A1));
    match execute_tx(client, nonce, aeurc, approve_aeurc, U256::zero(), 100000, "Arc FX Approve aEURC").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve aEURC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client, nonce, router,
        calldata::build_add_liquidity(
            agbpc, aeurc,
            U256::from(LP_AGBPC_AEURC_A0),
            U256::from(LP_AGBPC_AEURC_A1),
            to,
        ),
        U256::zero(), 600000, "Arc FX Add LP aGBPC/aEURC",
    ).await {
        Ok(n) => { nonce = n; lg("Arc FX add LP aGBPC/aEURC completed"); }
        Err(e) => lr(&format!("Arc FX Add LP aGBPC/aEURC failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    if can_claim(client, checkin, calldata::build_checkin()).await {
        match execute_tx(client, nonce, checkin, calldata::build_checkin(), U256::zero(), 300000, "Arc FX Check In").await {
            Ok(n) => { nonce = n; lg("Arc FX check in completed"); }
            Err(e) => lr(&format!("Arc FX Check In failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    } else {
        ly("Arc FX already checked in today, skipping");
    }

    let approve_usdc_stake = calldata::build_approve(stake, U256::from(STAKE_AMOUNT));
    match execute_tx(client, nonce, usdc, approve_usdc_stake, U256::zero(), 100000, "Arc FX Approve USDC for Stake").await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Arc FX Approve USDC for Stake failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(
        client, nonce, stake,
        calldata::build_stake(U256::from(STAKE_AMOUNT)),
        U256::zero(), 300000, "Arc FX Stake USDC",
    ).await {
        Ok(n) => { nonce = n; lg("Arc FX stake USDC completed"); }
        Err(e) => lr(&format!("Arc FX Stake USDC failed: {}", e)),
    }

    lg("Arc FX category execution completed");
    Ok(nonce)
}