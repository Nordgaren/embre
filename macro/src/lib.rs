#![doc = include_str!("../README.md")]
use embre_core::aes::{include_bytes_aes_impl, include_str_aes_impl};
use embre_core::xor::{include_bytes_xor_impl, include_str_xor_impl};
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_str_xor(input: TokenStream) -> TokenStream {
    include_str_xor_impl(input)
}

#[proc_macro]
pub fn include_bytes_xor(input: TokenStream) -> TokenStream {
    include_bytes_xor_impl(input)
}

#[proc_macro]
pub fn include_str_aes(input: TokenStream) -> TokenStream {
    include_str_aes_impl(input.into()).into()
}

#[proc_macro]
pub fn include_bytes_aes(input: TokenStream) -> TokenStream {
    include_bytes_aes_impl(input.into()).into()
}
