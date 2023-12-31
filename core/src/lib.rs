#![doc = include_str!("../README.md")]
#[cfg(feature = "aes")]
pub mod aes;
mod literal_bytes;
pub mod xor;
