use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_swap_exact_eth_for_tokens(
    amount_out_min: U256,
    path: Vec<Address>,
    to: Address,
    deadline: U256,
) -> Bytes {
    let mut data = sel4("7ff36ab5");
    let encoded = encode(&[
        Token::Uint(amount_out_min),
        Token::Array(path.into_iter().map(Token::Address).collect()),
        Token::Address(to),
        Token::Uint(deadline),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_add_liquidity_eth(
    token: Address,
    amount_token_desired: U256,
    amount_token_min: U256,
    amount_eth_min: U256,
    to: Address,
    deadline: U256,
) -> Bytes {
    let mut data = sel4("f305d719");
    let encoded = encode(&[
        Token::Address(token),
        Token::Uint(amount_token_desired),
        Token::Uint(amount_token_min),
        Token::Uint(amount_eth_min),
        Token::Address(to),
        Token::Uint(deadline),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}