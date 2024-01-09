use crate::literal_bytes::LitBytes;
use quote::quote;
use std::fs;
use syn::__private::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, LitStr};

pub(crate) struct StringArgs {
    string: String,
    key: Vec<u8>,
}
impl Parse for StringArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let string = input.parse::<LitStr>()?.value();
        match input.parse::<Comma>() {
            Ok(_) => {
                let key = input.parse::<LitBytes>()?.get_bytes();
                if string.len() != key.len() {
                    panic!(
                        "String and Key length differ string len: {} key len: {}",
                        string.len(),
                        key.len()
                    )
                }
                if !input.is_empty() {
                    let err = input.error("Expected end of input");
                    return Err(err);
                }
                Ok(StringArgs { string, key })
            }
            Err(_) => Ok(StringArgs {
                key: embre_utils::generate_random_bytes(string.len()),
                string,
            }),
        }
    }
}
pub fn include_str_xor_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as StringArgs);

    let str = embre_crypt::xor::xor_bytes(args.string.as_bytes(), args.key.as_slice());
    let key = args.key;
    let len = str.len();
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#str , )* ];
            const KEY: [u8; #len] = [ #(#key , )* ];
            XORString::new(&BYTES, &KEY)
        }
    );
    TokenStream::from(q)
}
pub(crate) struct DataArgs {
    data: Vec<u8>,
    key: Vec<u8>,
}
impl Parse for DataArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let string = input.parse::<LitStr>()?.value();
        let data = fs::read(string).expect("Could not read file");
        match input.parse::<Comma>() {
            Ok(_) => {
                let lookahead = input.lookahead1();
                // Check if we have a file that has the bytes to xor the data with, or if the user provided a byte group
                let key = if lookahead.peek(syn::LitStr) {
                    fs::read(input.parse::<LitStr>()?.value()).expect("Could not read key file")
                } else {
                    input.parse::<LitBytes>()?.get_bytes()
                };

                if data.len() != key.len() {
                    panic!(
                        "Data and Key length differ string len: {} key len: {}",
                        data.len(),
                        key.len()
                    )
                }
                if !input.is_empty() {
                    let err = input.error("Expected end of input");
                    return Err(err);
                }
                Ok(DataArgs { data, key })
            }
            Err(_) => Ok(DataArgs {
                key: embre_utils::generate_random_bytes(data.len()),
                data,
            }),
        }
    }
}
pub fn include_bytes_xor_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DataArgs);
    let data = embre_crypt::xor::xor_bytes(&args.data[..], &args.key[..]);
    let key = args.key;
    let len = data.len();
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#data , )* ];
            const KEY: [u8; #len] = [ #(#key , )* ];
            XORData::new(&BYTES, &KEY)
        }
    );

    TokenStream::from(q)
}
