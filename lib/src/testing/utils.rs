#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        name.rsplit("::").nth(1).unwrap_or(name)
    }};
}

pub fn encode_utf16<const T: usize>(string: &str) -> [u16; T] {
    let mut bytes = [0; T];

    string
        .encode_utf16()
        .enumerate()
        .take(T)
        .for_each(|(index, byte)| bytes[index] = byte);

    bytes
}
