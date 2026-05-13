use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

pub fn build_approve(spender: Address, amount: U256) -> Bytes {
    let selector = hex::decode("095ea7b3").unwrap();
    let encoded = encode(&[Token::Address(spender), Token::Uint(amount)]);
    let mut data = selector;
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}

pub fn sel4(selector_hex: &str) -> Vec<u8> {
    let s = selector_hex.trim_start_matches("0x");
    hex::decode(&s[..8]).unwrap_or_default()
}
