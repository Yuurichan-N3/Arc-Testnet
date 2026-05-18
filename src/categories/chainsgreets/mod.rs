pub mod calldata;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, TransactionRequest, U256};
use std::str::FromStr;

use crate::logger::{lg, lr, ly};
use crate::rpc::{execute_tx, Client};

const GM_CONTRACT: &str = "0xa5c8e9bfC260B64134F1beabc49429adf5bd0C77";
const GN_CONTRACT: &str = "0x73A31cf8B2257A9391C5a915E5697b5DC0A05e23";
const DEPLOY_CONTRACT: &str = "0x7357ACd2ABe856D24A33E13ca3e11e1D9f49C687";
const GM_GN_VALUE: u64 = 12_300_000_000_000_000;

async fn refresh_nonce(client: &Client, fallback: U256) -> U256 {
    client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or(fallback)
}

async fn already_done(client: &Client, contract: Address, data: ethers::types::Bytes) -> bool {
    let tx = TransactionRequest::new()
        .from(client.address())
        .to(contract)
        .data(data)
        .value(U256::from(GM_GN_VALUE));
    client.call(&tx.into(), None).await.is_err()
}

pub async fn run(client: &Client, mut nonce: U256) -> Result<U256> {
    ly("Chainsgreets category execution started");

    let gm = Address::from_str(GM_CONTRACT).unwrap();
    let gn = Address::from_str(GN_CONTRACT).unwrap();
    let deployer = Address::from_str(DEPLOY_CONTRACT).unwrap();
    let value = U256::from(GM_GN_VALUE);

    if already_done(client, gm, calldata::build_gm()).await {
        ly("Chainsgreets GM already said today, skipping");
    } else {
        match execute_tx(client, nonce, gm, calldata::build_gm(), value, 200000, "Chainsgreets Say GM").await {
            Ok(n) => { nonce = n; lg("Chainsgreets say GM completed"); }
            Err(e) => lr(&format!("Chainsgreets Say GM failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    }

    if already_done(client, gn, calldata::build_gn()).await {
        ly("Chainsgreets GN already said today, skipping");
    } else {
        match execute_tx(client, nonce, gn, calldata::build_gn(), value, 200000, "Chainsgreets Say GN").await {
            Ok(n) => { nonce = n; lg("Chainsgreets say GN completed"); }
            Err(e) => lr(&format!("Chainsgreets Say GN failed: {}", e)),
        }
        nonce = refresh_nonce(client, nonce).await;
    }

    match execute_tx(client, nonce, deployer, calldata::build_deploy_simple(), U256::zero(), 300000, "Chainsgreets Deploy Simple").await {
        Ok(n) => { nonce = n; lg("Chainsgreets simple contract deployed"); }
        Err(e) => lr(&format!("Chainsgreets Deploy Simple failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(client, nonce, deployer, calldata::build_deploy_token(), U256::zero(), 500000, "Chainsgreets Deploy Token").await {
        Ok(n) => { nonce = n; lg("Chainsgreets token deployed"); }
        Err(e) => lr(&format!("Chainsgreets Deploy Token failed: {}", e)),
    }
    nonce = refresh_nonce(client, nonce).await;

    match execute_tx(client, nonce, deployer, calldata::build_deploy_nft(), U256::zero(), 700000, "Chainsgreets Deploy NFT").await {
        Ok(n) => { nonce = n; lg("Chainsgreets NFT deployed"); }
        Err(e) => lr(&format!("Chainsgreets Deploy NFT failed: {}", e)),
    }

    lg("Chainsgreets category execution completed");
    Ok(nonce)
}