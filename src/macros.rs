#[macro_export]
macro_rules! include_cstr {
    ($path:literal $(,)?) => {{
        const VALUE: &'static ::core::ffi::CStr = const {
            match ::core::ffi::CStr::from_bytes_with_nul(concat!(include_str!($path), "\0").as_bytes()) {
                Ok(value) => value,
                Err(_) => panic!(concat!("interior NUL byte(s) in `", $path, "`")),
            }
        };
        VALUE
    }};
}
