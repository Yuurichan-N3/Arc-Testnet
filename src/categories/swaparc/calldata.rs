use ethers::abi::{encode, Token};
use ethers::types::{Bytes, U256};

use crate::calldata::sel4;

pub fn build_swap(i: u64, j: u64, dx: u64) -> Bytes {
    let mut data = sel4("9d9892cd");
    data.extend_from_slice(&encode(&[
        Token::Uint(U256::from(i)),
        Token::Uint(U256::from(j)),
        Token::Uint(U256::from(dx)),
    ]));
    Bytes::from(data)
}

pub fn build_add_liquidity(amount0: u64, amount1: u64) -> Bytes {
    let mut data = sel4("4de59aa3");
    data.extend_from_slice(&encode(&[Token::Array(vec![
        Token::Uint(U256::from(amount0)),
        Token::Uint(U256::from(amount1)),
    ])]));
    Bytes::from(data)
}