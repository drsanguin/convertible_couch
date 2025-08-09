#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        name.rsplit("::").nth(1).unwrap_or(name)
    }};
}
