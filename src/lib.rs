#![doc = include_str!("../README.md")]
#![allow(unused)]
pub use embre_macro::include_bytes_aes;
pub use embre_macro::include_bytes_xor;
pub use embre_macro::include_str_aes;
pub use embre_macro::include_str_xor;

pub mod aes;
pub mod embedded_resource;
pub(crate) mod util;
pub mod xor;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;
