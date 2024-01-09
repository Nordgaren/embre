#![doc = include_str!("../README.md")]
pub mod config;
pub mod resource;
pub mod resource_builder;
pub mod util;

pub(crate) fn make_const_name(string: &str) -> String {
    let underscores = [" ", ",", "."];
    let mut const_name = string.to_uppercase();

    for pattern in underscores {
        const_name = const_name.replace(pattern, "_")
    }

    let one: Vec<char> = ('!'..',').collect();
    let two: Vec<char> = (':'..'A').collect();
    let three: Vec<char> = ('['..'_').collect();
    let four: Vec<char> = ('{'..='~').collect();

    let delete = ['\0', '!', '\"', '-', '/', '`'];

    for pattern in delete
        .iter()
        .chain(one.iter())
        .chain(two.iter())
        .chain(three.iter())
        .chain(four.iter())
    {
        const_name = const_name.replace(*pattern, "")
    }

    const_name.replace("__", "_")
}


#[macro_export]
macro_rules! build_println {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}
