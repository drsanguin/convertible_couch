pub fn encode_utf16<const T: usize>(string: &str) -> [u16; T] {
    let mut bytes = [0; T];
    let string_as_utf16: Vec<u16> = string.encode_utf16().collect();

    for (pos, e) in string_as_utf16.iter().enumerate() {
        bytes[pos] = *e;
    }

    return bytes;
}
