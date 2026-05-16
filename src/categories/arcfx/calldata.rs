use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_claim() -> Bytes {
    Bytes::from(sel4("4e71d92d"))
}

pub fn build_approve(spender: Address, amount: U256) -> Bytes {
    let mut data = sel4("095ea7b3");
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_swap_exact_tokens_for_tokens(
    amount_in: U256,
    amount_out_min: U256,
    path: Vec<Address>,
    to: Address,
    deadline: U256,
) -> Bytes {
    let mut data = sel4("38ed1739");
    let encoded = encode(&[
        Token::Uint(amount_in),
        Token::Uint(amount_out_min),
        Token::Array(path.into_iter().map(Token::Address).collect()),
        Token::Address(to),
        Token::Uint(deadline),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_add_liquidity(
    token0: Address,
    token1: Address,
    amount0: U256,
    amount1: U256,
    to: Address,
) -> Bytes {
    let mut data = sel4("4b2fc7bb");
    let encoded = encode(&[
        Token::Address(token0),
        Token::Address(token1),
        Token::Uint(amount0),
        Token::Uint(amount1),
        Token::Address(to),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_checkin() -> Bytes {
    let mut data = sel4("d9a59e33");
    let encoded = encode(&[Token::Address(Address::zero())]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_stake(token_id: U256) -> Bytes {
    let mut data = sel4("a694fc3a");
    let encoded = encode(&[Token::Uint(token_id)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}