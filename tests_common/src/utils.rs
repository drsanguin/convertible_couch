pub fn encode_utf16<const T: usize>(string: &str) -> [u16; T] {
    let mut bytes = [0; T];

    string
        .encode_utf16()
        .collect::<Vec<u16>>()
        .iter()
        .enumerate()
        .for_each(|(byte_index, byte)| bytes[byte_index] = *byte);

    bytes
}
