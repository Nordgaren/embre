#[cfg(test)]
mod tests {
    use embre_macro::{include_xor_bytes, include_xor_string};
    use embre_run::xor_data::XORData;
    use embre_run::xor_string::XORString;
    use std::fs;

    const XOR_DATA: XORData = include_xor_bytes!("_test/cargo.toml");
    const XOR_STRING: XORString = include_xor_string!("test string");

    #[test]
    fn comparison_operators() {
        let xor_string: XORString = XOR_DATA.into();
        // Reading the cargo.toml from this crate. Cargo test will build the exe in the _test directory, but the test will still run in the embre directory.
        let cargo_string = fs::read_to_string("cargo.toml").expect("Could not read cargo.toml");
        let cargo_vec = cargo_string.as_bytes().to_vec();

        assert_eq!(xor_string, cargo_string, "Could not compare XORString and String");
        assert_eq!(xor_string, cargo_vec, "Could not compare XORString and Vec<u8>");

        assert_eq!(XOR_DATA, cargo_vec, "Could not compare XORData and Vec<u8>");

        assert_eq!(XOR_STRING, "test string", "Could not compare XOR_STRING and &str");
    }
}
