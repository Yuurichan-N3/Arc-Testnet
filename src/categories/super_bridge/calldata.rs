use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes};

use crate::calldata::sel4;
use crate::constants::{
    SUPERBRIDGE_AMOUNT, SUPERBRIDGE_DEST_DOMAIN, SUPERBRIDGE_MAX_FEE,
    SUPERBRIDGE_MIN_FINALITY, SUPERBRIDGE_TAG_HEX, USDC_ARC_TOKEN,
};

pub fn build_deposit_for_burn(wallet_addr: Address) -> Bytes {
    let selector = sel4("8e0250ee");

    let mint_recipient = address_to_bytes32(wallet_addr);
    let destination_caller = [0u8; 32];
    let usdc = USDC_ARC_TOKEN.parse::<Address>().unwrap();

    let encoded = encode(&[
        Token::Uint(ethers::types::U256::from(SUPERBRIDGE_AMOUNT)),
        Token::Uint(ethers::types::U256::from(SUPERBRIDGE_DEST_DOMAIN)),
        Token::FixedBytes(mint_recipient.to_vec()),
        Token::Address(usdc),
        Token::FixedBytes(destination_caller.to_vec()),
        Token::Uint(ethers::types::U256::from(SUPERBRIDGE_MAX_FEE)),
        Token::Uint(ethers::types::U256::from(SUPERBRIDGE_MIN_FINALITY)),
    ]);

    let tag = hex::decode(SUPERBRIDGE_TAG_HEX).unwrap_or_default();

    let mut data = selector;
    data.extend_from_slice(&encoded);
    data.extend_from_slice(&tag);
    Bytes::from(data)
}

fn address_to_bytes32(addr: Address) -> [u8; 32] {
    let mut buf = [0u8; 32];
    buf[12..].copy_from_slice(addr.as_bytes());
    buf
}
