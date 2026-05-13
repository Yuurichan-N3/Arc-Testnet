use ethers::types::Bytes;

pub fn build_gm() -> Bytes {
    Bytes::from(hex::decode("84a3bb6b0000000000000000000000006f479f2c97e9aa666323abd6a8f3a8821bccb289").unwrap())
}

pub fn build_mint_badge() -> Bytes {
    Bytes::from(hex::decode("26092b83").unwrap())
}

pub fn build_deploy() -> Bytes {
    Bytes::from(hex::decode("775c300c").unwrap())
}