#![doc = include_str!("../README.md")]
#![allow(unused)]
use embre_macro::include_aes_bytes;
use embre_macro::include_aes_string;
use embre_macro::include_xor_bytes;
use embre_macro::include_xor_string;

pub mod aes;
mod embedded_resource;
pub(crate) mod util;
pub mod xor;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;
