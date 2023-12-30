#![doc = include_str!("../README.md")]
#[cfg(test)]
mod tests {
    use embre::aes::aes_string::AESString;
    use embre::xor::xor_data::XORData;
    use embre::xor::xor_string::XORString;
    use embre_macro::{
        include_aes_bytes, include_aes_string, include_xor_bytes, include_xor_string,
    };
    use std::fs;

    const XOR_DATA: XORData = include_xor_bytes!("_test/cargo.toml");
    const XOR_STRING: XORString = include_xor_string!("test string");

    #[test]
    fn xor_comparison_operators() {
        let xor_string: XORString = XOR_DATA.into();
        // Reading the cargo.toml from this crate. Cargo test will build the exe in the _test directory, but the test will still run in the embre directory.
        let cargo_string = fs::read_to_string("cargo.toml").expect("Could not read cargo.toml");
        let cargo_vec = cargo_string.as_bytes().to_vec();

        assert_eq!(
            xor_string, cargo_string,
            "Could not compare XORString and String"
        );
        assert_eq!(
            xor_string, cargo_vec,
            "Could not compare XORString and Vec<u8>"
        );

        assert_eq!(XOR_DATA, cargo_vec, "Could not compare XORData and Vec<u8>");

        assert_eq!(
            XOR_STRING, "test string",
            "Could not compare XOR_STRING and &str"
        );
    }

    #[test]
    fn xor_comparison_operators_rhs() {
        let xor_string: XORString = XOR_DATA.into();
        // Reading the cargo.toml from this crate. Cargo test will build the exe in the _test directory, but the test will still run in the embre directory.
        let cargo_string = fs::read_to_string("cargo.toml").expect("Could not read cargo.toml");
        let cargo_vec = cargo_string.as_bytes().to_vec();

        assert_eq!(
            cargo_string, xor_string,
            "Could not compare XORString and String RHS"
        );
        assert_eq!(
            cargo_vec, xor_string,
            "Could not compare XORString and Vec<u8> RHS"
        );

        assert_eq!(
            cargo_vec, XOR_DATA,
            "Could not compare XORData and Vec<u8> RHS"
        );

        assert_eq!(
            "test string", XOR_STRING,
            "Could not compare XOR_STRING and &str RHS"
        );
    }

    // Need to find a way to make a const version of AESResource.
    // const AES_DATA: AESData = include_xor_bytes!("_test/cargo.toml");
    #[test]
    fn aes_comparison_operators() {
        let aes_string = include_aes_string!("test string");
        assert!(
            aes_string == "test string",
            "Could not compare AES_STRING and &str 'test string'"
        );
        let long_aes_string: AESString = include_aes_bytes!("_test/src/lib.rs").into();
        assert!(
            long_aes_string == include_str!("lib.rs"),
            "Could not compare AES_STRING and &str from lib.rs"
        );
    }
    #[test]
    fn aes_comparison_operators_rhs() {
        let aes_string = include_aes_string!("test string");
        assert!(
            "test string" == aes_string,
            "Could not compare AES_STRING and &str 'test string'"
        );
        let long_aes_string: AESString = include_aes_bytes!("_test/src/lib.rs").into();
        assert!(
            include_str!("lib.rs") == long_aes_string,
            "Could not compare AES_STRING and &str from lib.rs"
        );
    }
}
