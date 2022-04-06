use hex::FromHexError;

pub fn decode_hex(hex: &str) -> Result<Vec<u8>, FromHexError> {
    match hex.strip_prefix("0x") {
        None => hex::decode(hex),
        Some(stripped) => hex::decode(stripped),
    }
}
