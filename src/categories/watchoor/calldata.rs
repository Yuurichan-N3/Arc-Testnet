use ethers::abi::{encode, Token};
use ethers::types::{Bytes, U256};

use crate::calldata::sel4;

pub fn build_gm() -> Bytes {
    let mut data = sel4("92d0214c");
    data.extend_from_slice(&encode(&[Token::Uint(U256::zero())]));
    Bytes::from(data)
}

pub fn build_gn() -> Bytes {
    let mut data = sel4("00f76b87");
    data.extend_from_slice(&encode(&[Token::Uint(U256::zero())]));
    Bytes::from(data)
}

pub fn build_deploy_nft(name: &str, symbol: &str) -> Bytes {
    let mut data = sel4("f399e81c");
    data.extend_from_slice(&encode(&[
        Token::String(name.to_string()),
        Token::String(symbol.to_string()),
        Token::Uint(U256::zero()),
    ]));
    Bytes::from(data)
}

pub fn build_deploy_erc20(name: &str, symbol: &str) -> Bytes {
    let mut data = sel4("64d346cd");
    data.extend_from_slice(&encode(&[
        Token::String(name.to_string()),
        Token::String(symbol.to_string()),
        Token::Uint(U256::zero()),
    ]));
    Bytes::from(data)
}

pub fn build_deploy_counter() -> Bytes {
    let mut data = sel4("acbd0c47");
    data.extend_from_slice(&encode(&[Token::Uint(U256::zero())]));
    Bytes::from(data)
}
