use ethers::types::Bytes;

pub fn build_checkin() -> Bytes {
    Bytes::from(vec![0xfc, 0x12, 0x35, 0xbb])
}

pub fn build_deploy_simple(bytecode: &[u8]) -> Bytes {
    Bytes::from(bytecode.to_vec())
}