#![doc = include_str!("../README.md")]
#![allow(unused)]

#[cfg(feature = "aes")]
pub use embre_macro::{include_bytes_aes, include_str_aes};
pub use embre_macro::{include_bytes_xor, include_str_xor};
use std::fmt::Error;
use std::string::FromUtf8Error;
#[cfg(feature = "aes")]
pub mod aes;
pub mod embedded_resource;
pub mod xor;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;

#[inline(always)]
pub(crate) fn common_string_fmt(
    f: &mut std::fmt::Formatter<'_>,
    str_result: Result<String, FromUtf8Error>,
) -> std::fmt::Result {
    let str = match str_result {
        Ok(s) => s,
        Err(_) => return Err(Error),
    };
    write!(f, "{}", str)
}
