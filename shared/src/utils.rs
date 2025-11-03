/// Encodes one byte into 2 ascii-encoded hex digits.
#[inline(always)]
pub fn hex_encode(char: u8) -> [u8; 2] {
    let char_1 = char >> 4;
    let char_2 = char & 0x0F;
    [
        match char_1 {
            0x0..=0x9 => char_1 + b'0',
            0xA..=0xF => char_1 - 0xA + b'A',
            _ => unreachable!(),
        },
        match char_2 {
            0x0..=0x9 => char_2 + b'0',
            0xA..=0xF => char_2 - 0xA + b'A',
            _ => unreachable!(),
        },
    ]
}
