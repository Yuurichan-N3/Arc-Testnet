use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_register_username(username: &str) -> Bytes {
    let mut data = sel4("36a94134");
    let encoded = encode(&[Token::String(username.to_string())]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_increase_allowance(spender: Address, amount: U256) -> Bytes {
    let mut data = sel4("39509351");
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_bridge(
    amount: U256,
    recipient: Address,
    token: Address,
    bridge_contract: Address,
    dest_chain_id: u32,
    nonce: u32,
) -> Bytes {
    let mut data = sel4("d0d4229a");

    let mut recipient_bytes32 = [0u8; 32];
    recipient_bytes32[12..].copy_from_slice(recipient.as_bytes());
    let zero_bytes32 = [0u8; 32];

    let encoded = encode(&[Token::Tuple(vec![
        Token::Uint(amount),
        Token::Uint(U256::zero()),
        Token::Uint(U256::zero()),
        Token::FixedBytes(recipient_bytes32.to_vec()),
        Token::FixedBytes(zero_bytes32.to_vec()),
        Token::Address(token),
        Token::Address(bridge_contract),
        Token::Uint(U256::from(dest_chain_id)),
        Token::Uint(U256::from(nonce)),
    ])]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

fn pad32(val: U256) -> [u8; 32] {
    let mut buf = [0u8; 32];
    val.to_big_endian(&mut buf);
    buf
}