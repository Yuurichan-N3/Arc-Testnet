use ethers::abi::{encode, Token};
use ethers::types::Bytes;

use crate::calldata::sel4;

pub fn build_say_gm(message: &str) -> Bytes {
    let selector = sel4("8cb09282");
    let encoded = encode(&[Token::String(message.to_string())]);
    let mut data = selector;
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_deploy_contract() -> Bytes {
    let selector = sel4("2c39c058");
    let encoded = encode(&[Token::Uint(ethers::types::U256::zero())]);
    let mut data = selector;
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}
