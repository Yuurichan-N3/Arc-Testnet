pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, TransactionRequest, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, silent_approve, Client};

const FAUCET: &str = "0xEd2520116C9e6F2517daa20Eb7FFF4EeA5bE6847";
const LP_CONTRACT: &str = "0x49f9636FE15883e16d5E356A4eA08C9Fe6BC219B";
const USDC: &str = "0x3600000000000000000000000000000000000000";
const CAT: &str = "0xc3328be246C5DB1a2EBA7d0533e275a0a7249834";
const DARC: &str = "0x8959ed0d7220e1baa445106f48829df0bf1e5f83";
const PANDA: &str = "0x48ff1ccb0f75e5a8c732b6c10ffc8f5df6ef5311";

const LP_USDC_CAT_A: u64 = 1000;
const LP_USDC_CAT_B: u64 = 44_293_000_000_000_000;
const LP_USDC_DARC_A: u64 = 1000;
const LP_USDC_DARC_B: u64 = 15_624_000_000_000_000;
const LP_USDC_PANDA_A: u64 = 100;
const LP_USDC_PANDA_B: u64 = 3_120_000_000_000_000;

async fn preflight_claim(client: &Client) -> bool {
    let faucet: Address = FAUCET.parse().unwrap();
    let tx = TransactionRequest::new()
        .from(client.address())
        .to(faucet)
        .data(calldata::build_claim())
        .value(U256::zero());
    match client.call(&tx.into(), None).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Flow On Arc category execution started");

    let faucet = Address::from_str(FAUCET).unwrap();
    let lp_contract = Address::from_str(LP_CONTRACT).unwrap();
    let usdc = Address::from_str(USDC).unwrap();
    let cat = Address::from_str(CAT).unwrap();
    let darc = Address::from_str(DARC).unwrap();
    let panda = Address::from_str(PANDA).unwrap();

    let claim_ok = preflight_claim(client).await;
    if !claim_ok {
        ly("Flow On Arc faucet already claimed this period, skipping claim");
    } else {
        match execute_tx(
            client,
            nonce,
            faucet,
            calldata::build_claim(),
            U256::zero(),
            200000,
            "Flow On Arc Faucet Claim",
        )
        .await
        {
            Ok(n) => {
                nonce = n;
                lg("Flow On Arc faucet claim completed");
            }
            Err(e) => {
                lr(&format!("Flow On Arc faucet claim failed: {}", e));
            }
        }
    }

    match silent_approve(client, nonce, usdc, lp_contract, U256::MAX).await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc approve USDC failed: {}", e)),
    }

    match silent_approve(client, nonce, cat, lp_contract, U256::MAX).await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc approve CAT failed: {}", e)),
    }

    match execute_tx(
        client,
        nonce,
        lp_contract,
        calldata::build_add_liquidity(
            usdc,
            cat,
            U256::from(LP_USDC_CAT_A),
            U256::from(LP_USDC_CAT_B),
        ),
        U256::zero(),
        300000,
        "Flow On Arc Add LP USDC/CAT",
    )
    .await
    {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc Add LP USDC/CAT failed: {}", e)),
    }

    match silent_approve(client, nonce, darc, lp_contract, U256::MAX).await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc approve DARC failed: {}", e)),
    }

    match execute_tx(
        client,
        nonce,
        lp_contract,
        calldata::build_add_liquidity(
            usdc,
            darc,
            U256::from(LP_USDC_DARC_A),
            U256::from(LP_USDC_DARC_B),
        ),
        U256::zero(),
        300000,
        "Flow On Arc Add LP USDC/DARC",
    )
    .await
    {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc Add LP USDC/DARC failed: {}", e)),
    }

    match silent_approve(client, nonce, panda, lp_contract, U256::MAX).await {
        Ok(n) => nonce = n,
        Err(e) => lr(&format!("Flow On Arc approve PANDA failed: {}", e)),
    }

    match execute_tx(
        client,
        nonce,
        lp_contract,
        calldata::build_add_liquidity(
            usdc,
            panda,
            U256::from(LP_USDC_PANDA_A),
            U256::from(LP_USDC_PANDA_B),
        ),
        U256::zero(),
        300000,
        "Flow On Arc Add LP USDC/PANDA",
    )
    .await
    {
        Ok(n) => {
            nonce = n;
            lg("Flow On Arc add liquidity execution completed");
        }
        Err(e) => lr(&format!("Flow On Arc Add LP USDC/PANDA failed: {}", e)),
    }

    Ok(nonce)
}