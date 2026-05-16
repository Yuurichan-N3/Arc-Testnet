use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_approve(spender: Address, amount: U256) -> Bytes {
    let mut data = sel4("095ea7b3");
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_swap(
    token_in: Address,
    token_out: Address,
    amount_in: U256,
    min_amount_out: U256,
    max_slippage: U256,
) -> Bytes {
    let mut data = sel4("7a950f99");
    let encoded = encode(&[
        Token::Address(token_in),
        Token::Address(token_out),
        Token::Uint(amount_in),
        Token::Uint(min_amount_out),
        Token::Uint(max_slippage),
    ]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
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

pub fn build_increase_allowance(spender: Address, amount: U256) -> Bytes {
    let mut data = sel4("39509351");
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn build_bridge(
    amount: U256,
    min_amount: U256,
    recipient: Address,
    token: Address,
    bridge_contract: Address,
    dest_chain_id: u32,
    nonce: u32,
) -> Bytes {
    let mut data = sel4("513e1175");

    let mut recipient_bytes32 = [0u8; 32];
    recipient_bytes32[12..].copy_from_slice(recipient.as_bytes());

    let zero_bytes32 = [0u8; 32];

    let hook_data_str = b"cctp-forward";
    let hook_data_len = hook_data_str.len(); // 12
    let mut hook_data_padded = [0u8; 32];
    hook_data_padded[..hook_data_len].copy_from_slice(hook_data_str);

    let mut encoded = Vec::new();

    encoded.extend_from_slice(&pad32(amount));
    encoded.extend_from_slice(&pad32(min_amount));
    encoded.extend_from_slice(&pad32(U256::zero()));
    encoded.extend_from_slice(&recipient_bytes32);
    encoded.extend_from_slice(&zero_bytes32);
    encoded.extend_from_slice(&pad32(U256::from_big_endian(token.as_bytes())));
    encoded.extend_from_slice(&pad32(U256::from_big_endian(bridge_contract.as_bytes())));
    encoded.extend_from_slice(&pad32(U256::from(dest_chain_id)));
    encoded.extend_from_slice(&pad32(U256::from(nonce)));

    let hook_offset = U256::from(10u64 * 32u64);
    encoded.extend_from_slice(&pad32(hook_offset));

    encoded.extend_from_slice(&pad32(U256::from(32u64)));
    encoded.extend_from_slice(&hook_data_padded);

    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

fn pad32(val: U256) -> [u8; 32] {
    let mut buf = [0u8; 32];
    val.to_big_endian(&mut buf);
    buf
}