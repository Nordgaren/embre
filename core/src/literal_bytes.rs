use proc_macro2::Group;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::LitInt;
type TokenStream2 = proc_macro2::token_stream::TokenStream;

pub struct LitBytes {
    lit_ints: Vec<LitInt>,
}

impl LitBytes {
    pub fn get_bytes(self) -> Vec<u8> {
        self.lit_ints
            .iter()
            .map(|b| b.base10_parse().unwrap())
            .collect()
    }
}
impl Parse for LitBytes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group: Group = input.parse()?;

        let mut is_comma = false;

        let mut numbers = Vec::new();

        for t in group.stream() {
            if is_comma {
                let comma: TokenStream2 = syn::parse2(t.to_token_stream())?;
                assert_eq!(comma.to_string(), ",");
            } else {
                let number: LitInt = syn::parse2(t.to_token_stream())?;
                numbers.push(number);
            }

            is_comma = !is_comma;
        }

        Ok(LitBytes { lit_ints: numbers })
    }
}
