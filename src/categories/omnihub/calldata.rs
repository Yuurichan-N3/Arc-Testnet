use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

const ZERO_ADDR: &str = "0x0000000000000000000000000000000000000000";

pub fn build_mint_shrimp(quantity: u64) -> Bytes {
    let zero: Address = ZERO_ADDR.parse().unwrap();
    let mut data = sel4("a25ffea8");
    let encoded = encode(&[
        Token::Uint(U256::zero()),
        Token::Uint(U256::from(quantity)),
        Token::Address(zero),
        Token::Array(vec![]),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}