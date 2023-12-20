use embre_core::aes::{include_aes_bytes_impl, include_aes_string_impl};
use embre_core::xor::{include_xor_bytes_impl, include_xor_string_impl};
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_xor_string(input: TokenStream) -> TokenStream {
    include_xor_string_impl(input)
}

#[proc_macro]
pub fn include_xor_bytes(input: TokenStream) -> TokenStream {
    include_xor_bytes_impl(input)
}

#[proc_macro]
pub fn include_aes_string(input: TokenStream) -> TokenStream {
    include_aes_string_impl(input.into()).into()
}

#[proc_macro]
pub fn include_aes_bytes(input: TokenStream) -> TokenStream {
    include_aes_bytes_impl(input.into()).into()
}
