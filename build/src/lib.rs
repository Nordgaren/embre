pub mod config;
mod resource;
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
