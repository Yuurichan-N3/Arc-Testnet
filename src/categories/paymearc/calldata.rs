use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_approve(spender: Address, amount: U256) -> Bytes {
    let mut data = sel4("095ea7b3");
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_pay(
    token: Address,
    from: Address,
    amount: U256,
    ref_bytes: [u8; 32],
) -> Bytes {
    let mut data = sel4("3e8bca68");
    let encoded = encode(&[
        Token::Address(token),
        Token::Address(from),
        Token::Uint(amount),
        Token::FixedBytes(ref_bytes.to_vec()),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}