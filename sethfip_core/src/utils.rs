use hex::FromHexError;

/// Takes a string representation of hexadecimal numbers. The string can start with '0x'. Returns
/// decoded bytes.
///
/// # Note
/// The number of characters must be divisible by 2.
///
/// # Example
/// ```
/// # use sethfip_core::decode_hex;
///
/// let decoded = decode_hex("00").unwrap();
/// assert_eq!(decoded, [0]);
/// let decoded = decode_hex("0x99").unwrap();
/// assert_eq!(decoded, [9 * 16 + 9]);
/// let decoded = decode_hex("0xA7").unwrap();
/// assert_eq!(decoded, [10 * 16 + 7]);
/// let decoded = decode_hex("Ba").unwrap();
/// assert_eq!(decoded, [11 * 16 + 10]);
/// ```
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, FromHexError> {
    match hex.strip_prefix("0x") {
        None => hex::decode(hex),
        Some(stripped) => hex::decode(stripped),
    }
}
