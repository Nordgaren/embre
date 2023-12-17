use crate::literal_bytes::LitBytes;
use embedded_resources_build::util::xor_bytes;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::fs;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, LitStr};
use embedded_resources_crypt::aes::aes_encrypt_bytes;

pub(crate) struct StringArgs {
    string: String,
    key: Vec<u8>,
    iv: Vec<u8>,
}
impl Parse for StringArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let string = input.parse::<LitStr>()?.value();
        if input.is_empty() {
            return Ok(StringArgs {
                string,
                key: vec![],
                iv: vec![],
            })
        }
        // Check for key
        let key;
        match input.parse::<Comma>() {
            Ok(_) => {
                key = input.parse::<LitBytes>()?.get_bytes();
                if input.is_empty() {
                    return Ok(StringArgs { string, key, iv: vec![] });
                }
            }
            Err(_) => {
                let err = input.error("Expected comma after string.");
                return Err(err);
            }
        }
        // Check for iv
        let iv;
        match input.parse::<Comma>() {
            Ok(_) => {
                iv = input.parse::<LitBytes>()?.get_bytes();
                if input.is_empty() {
                    return Ok(StringArgs { string, key, iv });
                }
            }
            Err(_) => {
                let err = input.error("Expected comma after key.");
                return Err(err);
            }
        }
        let err = input.error("Expected end of input.");
        Err(err)
    }
}
pub(crate) fn include_aes_string_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as StringArgs);
    let iv = if args.iv.is_empty() {
        None
    } else {
        Some(&args.iv[..])
    };
    args.key.into_iter();
    let str = aes_encrypt_bytes(args.string.as_bytes(), args.key.as_slice(), iv).expect("Could not AES encrypt bytes");
    let key = args.key;
    let len = str.len();
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#str , )* ];
            const KEY: [u8; #len] = [ #(#key , )* ];
            AESString::new(&BYTES, &KEY)
        }
    );
    TokenStream::from(q)
}
pub(crate) struct DataArgs {
    data: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
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
                Ok(DataArgs {  data, key, iv: vec![] })
            }
            Err(_) => Ok(DataArgs {
                key: embedded_resources_build::util::generate_random_bytes(data.len()),
                data,
                iv: vec![],
            }),
        }
    }
}
pub(crate) fn include_aes_bytes_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DataArgs);
    let iv = if args.iv.is_empty() {
        None
    } else {
        Some(&args.iv[..])
    };
    let data = aes_encrypt_bytes(&args.data[..], &args.key[..], iv).expect("Could not encrypt bytes");
    let key = args.key;
    let len = data.len();
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#data , )* ];
            const KEY: [u8; #len] = [ #(#key , )* ];
            AESData::new(&BYTES, &KEY)
        }
    );

    TokenStream::from(q)
}
