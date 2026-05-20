use ethers::types::Bytes;

use crate::calldata::sel4;

pub fn build_gm() -> Bytes {
    Bytes::from(sel4("25406903"))
}

pub fn build_gn() -> Bytes {
    Bytes::from(sel4("0ec21ff2"))
}