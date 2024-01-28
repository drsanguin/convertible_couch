#[macro_export]
macro_rules! func {
    () => {{
        use convertible_couch_tests_common::fuzzing::Fuzzer;
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        &name[19..name.len() - 3]
    }};
}

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
