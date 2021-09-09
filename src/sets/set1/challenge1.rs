use base64;
use hex;

pub fn hex2base64(hexstr: &str) -> String {
    let decoded_bytes: Vec<u8> = hex::decode(&hexstr).expect("Could not return Vec.");
    base64::encode_config(&decoded_bytes, base64::STANDARD)
}   