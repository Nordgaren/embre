#![doc = include_str!("../README.md")]
use embre_core::aes::{include_aes_bytes_impl, include_aes_str_impl};
use embre_core::xor::{include_xor_bytes_impl, include_xor_str_impl};
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_xor_str(input: TokenStream) -> TokenStream {
    include_xor_str_impl(input)
}

#[proc_macro]
pub fn include_xor_bytes(input: TokenStream) -> TokenStream {
    include_xor_bytes_impl(input)
}

#[proc_macro]
pub fn include_aes_str(input: TokenStream) -> TokenStream {
    include_aes_str_impl(input.into()).into()
}

#[proc_macro]
pub fn include_aes_bytes(input: TokenStream) -> TokenStream {
    include_aes_bytes_impl(input.into()).into()
}
