use ethers::types::{Address, Bytes};

pub fn build_claim(receiver: Address) -> Bytes {
    let selector = hex::decode("57bc3d78").unwrap();

    let mut w_receiver = [0u8; 32];
    w_receiver[12..].copy_from_slice(receiver.as_bytes());

    let w_token_id = [0u8; 32];

    let mut w_quantity = [0u8; 32];
    w_quantity[31] = 1;

    let currency_bytes =
        hex::decode("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    let mut w_currency = [0u8; 32];
    w_currency[12..].copy_from_slice(&currency_bytes);

    let price: u64 = 0xb7bc7e60fd6a;
    let mut w_price = [0u8; 32];
    w_price[24..].copy_from_slice(&price.to_be_bytes());

    let mut w_proof_offset = [0u8; 32];
    w_proof_offset[30] = 0x00;
    w_proof_offset[31] = 0xe0;

    let mut w_data_offset = [0u8; 32];
    w_data_offset[30] = 0x01;
    w_data_offset[31] = 0x80;

    let mut w_arr_inner_offset = [0u8; 32];
    w_arr_inner_offset[31] = 0x80;

    let w_max_quantity = [0u8; 32];

    let w_max_price = [0xffu8; 32];

    let w_allowlist_currency = [0u8; 32];

    let w_arr_len = [0u8; 32];

    let w_data_len = [0u8; 32];

    let mut out = Vec::with_capacity(4 + 32 * 14);
    out.extend_from_slice(&selector);
    out.extend_from_slice(&w_receiver);
    out.extend_from_slice(&w_token_id);
    out.extend_from_slice(&w_quantity);
    out.extend_from_slice(&w_currency);
    out.extend_from_slice(&w_price);
    out.extend_from_slice(&w_proof_offset);
    out.extend_from_slice(&w_data_offset);
    out.extend_from_slice(&w_arr_inner_offset);
    out.extend_from_slice(&w_max_quantity);
    out.extend_from_slice(&w_max_price);
    out.extend_from_slice(&w_allowlist_currency);
    out.extend_from_slice(&w_arr_len);
    out.extend_from_slice(&w_data_len);

    Bytes::from(out)
}