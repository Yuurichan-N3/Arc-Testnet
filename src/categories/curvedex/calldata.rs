use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

const ZERO_ADDR: &str = "0x0000000000000000000000000000000000000000";

fn addr11(a0: Address, a1: Address, a2: Address) -> Vec<Token> {
    let zero: Address = ZERO_ADDR.parse().unwrap();
    let mut v = vec![
        Token::Address(a0),
        Token::Address(a1),
        Token::Address(a2),
    ];
    for _ in 0..8 {
        v.push(Token::Address(zero));
    }
    v
}

fn mat4x5(first_row: [u64; 4]) -> Vec<Token> {
    let make_row = |row: [u64; 4]| -> Token {
        Token::FixedArray(row.iter().map(|&x| Token::Uint(U256::from(x))).collect())
    };
    let zero_row = make_row([0, 0, 0, 0]);
    vec![
        make_row(first_row),
        zero_row.clone(),
        zero_row.clone(),
        zero_row.clone(),
        zero_row,
    ]
}

pub fn build_exchange(
    route_a: Address,
    route_b: Address,
    route_c: Address,
    row: [u64; 4],
    amount_in: U256,
    min_out: U256,
) -> Bytes {
    let mut data = sel4("aad348a2");
    let encoded = encode(&[
        Token::FixedArray(addr11(route_a, route_b, route_c)),
        Token::FixedArray(mat4x5(row)),
        Token::Uint(amount_in),
        Token::Uint(min_out),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_add_liquidity(amounts: &[u64], min_mint: u64) -> Bytes {
    let mut data = sel4("b72df5de");
    let encoded = encode(&[
        Token::Array(amounts.iter().map(|&x| Token::Uint(U256::from(x))).collect()),
        Token::Uint(U256::from(min_mint)),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_stake(amount: U256) -> Bytes {
    let mut data = sel4("b6b55f25");
    let encoded = encode(&[Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}
