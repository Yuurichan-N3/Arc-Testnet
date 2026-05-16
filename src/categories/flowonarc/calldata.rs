use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_claim() -> Bytes {
    Bytes::from(sel4("4e71d92d"))
}

pub fn build_add_liquidity(
    token_a: Address,
    token_b: Address,
    amount_a: U256,
    amount_b: U256,
) -> Bytes {
    let mut data = sel4("cf6c62ea");
    let encoded = encode(&[
        Token::Address(token_a),
        Token::Address(token_b),
        Token::Uint(amount_a),
        Token::Uint(amount_b),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}