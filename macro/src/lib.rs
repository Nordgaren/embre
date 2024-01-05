#![doc = include_str!("../README.md")]

#[cfg(feature = "aes")]
use embre_core::aes::{include_bytes_aes_impl, include_str_aes_impl};
use embre_core::xor::{include_bytes_xor_impl, include_str_xor_impl};
use proc_macro::TokenStream;

/// Embeds the string  and key as a static byte slice, and gives the users an XORString with the syntax
/// `XORString::new(&BYTES, &KEY)`.
///
/// The user can define their own XORString with XORString::new(data: &[u8], key: &[u8]) to deal with the string, or they can use the
/// `embre::xor::xor_string::XORString` provided in the crate.
///
/// # Arguments
///
/// * `input`: Can provide a string and an optional key in the array syntax style. They xor key must match the strings length,
/// but I may change that later to wrap the key value while XORing the target data.
///
/// returns: XORString
///
/// # Examples
///
/// ```
/// # struct XORString; // Fake type to keep linter from complaining.
/// # use embre_macro::include_str_xor;
/// const XOR_STRING: XORString = include_str_xor!("test string");
/// const XOR_STRING_WITH_KEY: XORString = include_str_xor!("test string", [10, 125, 40, 55, 100, 110, 40, 120, 250, 19, 103]);
/// ```
#[proc_macro]
pub fn include_str_xor(input: TokenStream) -> TokenStream {
    include_str_xor_impl(input)
}
/// Embeds the file and key as a static byte slice, and gives the users an XORString with the syntax
/// `XORData::new(&BYTES, &KEY)`.
///
/// The user can define their own XORData with XORData::new(data: &[u8], key: &[u8]) to deal with the string, or they can use the
/// `embre::xor::xor_data::XORData` provided in the crate.
///
/// # Arguments
///
/// * `input`: Can provide the path to a file and an optional key in the array syntax style. They xor key must match the strings length,
/// but I may change that later to wrap the key value while XORing the target data.
///
/// returns: XORString
///
/// # Examples
///
/// ```
/// # struct XORData; // Fake type to keep linter from complaining.
/// # use embre_macro::include_bytes_xor;
/// const XOR_STRING: XORData = include_bytes_xor!("P:/ath/to/file.bin");
/// const XOR_STRING_WITH_KEY: XORData = include_bytes_xor!("P:/ath/to/file.bin", [10, 125, 40, 55, 100, 110, 40]);
/// ```
#[proc_macro]
pub fn include_bytes_xor(input: TokenStream) -> TokenStream {
    include_bytes_xor_impl(input)
}

#[cfg(feature = "aes")]
#[proc_macro]
pub fn include_str_aes(input: TokenStream) -> TokenStream {
    include_str_aes_impl(input.into()).into()
}

#[cfg(feature = "aes")]
#[proc_macro]
pub fn include_bytes_aes(input: TokenStream) -> TokenStream {
    include_bytes_aes_impl(input.into()).into()
}
