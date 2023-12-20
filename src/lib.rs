#![doc = include_str!("../README.md")]
#![allow(unused)]
pub use embre_macro::include_aes_bytes;
pub use embre_macro::include_aes_string;
pub use embre_macro::include_xor_bytes;
pub use embre_macro::include_xor_string;

pub mod aes;
pub mod xor;
pub(crate) mod util;
mod embedded_resource;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;
