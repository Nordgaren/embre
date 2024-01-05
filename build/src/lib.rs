#![doc = include_str!("../README.md")]
pub mod config;
pub mod resource;
pub mod resource_builder;
#[cfg(test)]
mod tests;
pub mod util;

#[macro_export]
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}
