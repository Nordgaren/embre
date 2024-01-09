#![doc = include_str!("../README.md")]
#[cfg(feature = "aes")]
pub mod aes;
#[cfg(feature = "rsa")]
pub mod rsa;
pub mod xor;
