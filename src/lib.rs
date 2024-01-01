#![doc = include_str!("../README.md")]
#![allow(unused)]

#[cfg(feature = "aes")]
pub use embre_macro::{include_str_aes, include_bytes_aes};
pub use embre_macro::{include_str_xor, include_bytes_xor};
#[cfg(feature = "aes")]
pub mod aes;
pub mod embedded_resource;
pub(crate) mod util;
pub mod xor;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;
