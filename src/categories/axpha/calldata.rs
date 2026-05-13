use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

const TOKEN_USDC: &str = "0x911b4000d3422f482f4062a913885f7b035382df";
const VALUE_WEI: u64 = 100_000_000_000_000_000;

pub fn build_swap(token_out: Address, recipient: Address, deadline: u64) -> Bytes {
    let mut data = sel4("04a5baf1");

    let pad_u256 = |v: U256| -> [u8; 32] {
        let mut b = [0u8; 32];
        v.to_big_endian(&mut b);
        b
    };

    let pad_addr = |a: Address| -> [u8; 32] {
        let mut b = [0u8; 32];
        b[12..].copy_from_slice(a.as_bytes());
        b
    };

    let expected_out = U256::one();
    let min_out = U256::one();
    let value = U256::from(VALUE_WEI);

    let usdc: Address = TOKEN_USDC.parse().unwrap();

    data.extend_from_slice(&pad_u256(U256::from(0xc0u64)));
    data.extend_from_slice(&pad_addr(token_out));
    data.extend_from_slice(&pad_u256(min_out));
    data.extend_from_slice(&pad_u256(expected_out));
    data.extend_from_slice(&pad_addr(recipient));
    data.extend_from_slice(&pad_u256(U256::from(deadline)));
    data.extend_from_slice(&pad_u256(U256::one()));
    data.extend_from_slice(&pad_u256(U256::from(0x20u64)));
    data.extend_from_slice(&pad_u256(U256::one()));
    data.extend_from_slice(&pad_addr(usdc));
    data.extend_from_slice(&pad_addr(token_out));
    data.extend_from_slice(&pad_u256(value));
    data.extend_from_slice(&pad_u256(U256::zero()));
    data.extend_from_slice(&pad_u256(U256::from(0xc0u64)));
    data.extend_from_slice(&pad_u256(U256::zero()));

    Bytes::from(data)
}
