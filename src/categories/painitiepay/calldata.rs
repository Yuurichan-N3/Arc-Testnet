use ethers::abi::{encode, Token};
use ethers::types::{Address, Bytes, U256};

use crate::calldata::sel4;

pub fn build_transfer(to: Address, amount: U256) -> Bytes {
    let mut data = sel4("a9059cbb");
    let encoded = encode(&[Token::Address(to), Token::Uint(amount)]);
    data.extend_from_slice(&encoded);
    Bytes::from(data)
}