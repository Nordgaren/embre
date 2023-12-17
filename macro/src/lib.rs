mod xor;
mod literal_bytes;
mod aes;

use proc_macro::TokenStream;
use crate::aes::{include_aes_bytes_impl, include_aes_string_impl};
use crate::xor::{include_xor_bytes_impl, include_xor_string_impl};

#[proc_macro]
pub fn include_xor_string(input: TokenStream)  -> TokenStream {
    include_xor_string_impl(input)
}

#[proc_macro]
pub fn include_xor_bytes(input: TokenStream)  -> TokenStream {
    include_xor_bytes_impl(input)
}

#[proc_macro]
pub fn include_aes_string(input: TokenStream)  -> TokenStream {
    include_aes_string_impl(input)
}

#[proc_macro]
pub fn include_aes_bytes(input: TokenStream)  -> TokenStream {
    include_aes_bytes_impl(input)
}
