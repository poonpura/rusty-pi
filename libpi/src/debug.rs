/// Code to assist with debugging 

const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";

/// Converts `value` to a hexadecimal string representation and stores it in `buf`
pub fn u32_as_hex(value: u32, buffer: &mut [u8; 10]) -> &str {
    buffer[0] = b'0';
    buffer[1] = b'x';
    for i in 0..8 {
        buffer[9 - i] = HEX_DIGITS[(value >> (4 * i)) as usize & 0xF];
    }
    core::str::from_utf8(buffer).unwrap()
}