pub mod config;
pub mod resource_builder;
mod resource;
#[cfg(test)]
mod tests;
pub mod util;

#[macro_export]
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

