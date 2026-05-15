use ethers::abi::{encode, Token};
use ethers::types::{Bytes, U256};

use crate::calldata::sel4;

pub fn build_deposit() -> Bytes {
    Bytes::from(sel4("d0e30db0"))
}

pub fn build_withdraw(amount: U256) -> Bytes {
    let mut data = sel4("2e1a7d4d");
    let encoded = encode(&[Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}