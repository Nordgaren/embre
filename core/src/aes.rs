use crate::literal_bytes::LitBytes;
use embre_crypt::aes::{AESCrypter, DefaultAesCrypter};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse2, LitStr};

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
            });
        }
        // Check for key
        let key;
        match input.parse::<Comma>() {
            Ok(_) => {
                key = input.parse::<LitBytes>()?.get_bytes();
                if input.is_empty() {
                    return Ok(StringArgs {
                        string,
                        key,
                        iv: vec![],
                    });
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

pub fn include_aes_string_impl(input: TokenStream) -> TokenStream {
    let mut args = parse2::<StringArgs>(input)
        .expect("Could not parse StringArgs. Invalid arguments passed to include_aes_string!.");
    let crypter = DefaultAesCrypter::default();
    let cipher = crypter.get_cipher();

    let key = if args.key.is_empty() {
        args.key
            .extend(embre_build::util::generate_random_bytes(cipher.key_len()));
        &args.key[..]
    } else {
        &args.key[..]
    };

    let iv = if let Some(len) = cipher.iv_len() {
        if args.iv.is_empty() {
            args.iv
                .extend(embre_build::util::generate_random_bytes(len));
            Some(&args.iv[..])
        } else {
            Some(&args.iv[..])
        }
    } else {
        None
    };

    let str = crypter
        .aes_encrypt_bytes(args.string.as_bytes(), key, iv)
        .expect("Could not AES encrypt bytes");
    let len = str.len();

    let key = args.key;
    let key_len = cipher.key_len();

    let iv = args.iv;
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#str , )* ];
            const KEY: [u8; #key_len] = [ #(#key , )* ];
            const IV: Option<&'static [u8]> = Some(&[ #(#iv , )* ]);
            AESString::new(&BYTES, &KEY, IV)
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
        let lookahead = input.lookahead1();
        // Check if we have a file or just some bytes that need to be encrypted.
        let data = if lookahead.peek(syn::LitStr) {
            fs::read(input.parse::<LitStr>()?.value()).expect("Could not read file for encryption")
        } else {
            input.parse::<LitBytes>()?.get_bytes()
        };
        if input.is_empty() {
            return Ok(DataArgs {
                data,
                key: vec![],
                iv: vec![],
            });
        }
        // Check for key
        let key;
        match input.parse::<Comma>() {
            Ok(_) => {
                key = input.parse::<LitBytes>()?.get_bytes();
                if input.is_empty() {
                    return Ok(DataArgs {
                        data,
                        key,
                        iv: vec![],
                    });
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
                    return Ok(DataArgs { data, key, iv });
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

pub fn include_aes_bytes_impl(input: TokenStream) -> TokenStream {
    let mut args = parse2::<DataArgs>(input)
        .expect("Could not parse StringArgs. Invalid arguments passed to include_aes_bytes!.");
    let crypter = DefaultAesCrypter::default();
    let cipher = crypter.get_cipher();

    let key = if args.key.is_empty() {
        args.key
            .extend(embre_build::util::generate_random_bytes(cipher.key_len()));
        &args.key[..]
    } else {
        &args.key[..]
    };

    let iv = if let Some(len) = cipher.iv_len() {
        if args.iv.is_empty() {
            args.iv
                .extend(embre_build::util::generate_random_bytes(len));
            Some(&args.iv[..])
        } else {
            Some(&args.iv[..])
        }
    } else {
        None
    };

    let data = crypter
        .aes_encrypt_bytes(&args.data[..], key, iv)
        .expect("Could not encrypt bytes");
    let len = data.len();

    let key = args.key;
    let key_len = cipher.key_len();

    let iv = args.iv;
    let q = quote!(
        {
            const BYTES: [u8; #len] = [ #(#data , )* ];
            const KEY: [u8; #key_len] = [ #(#key , )* ];
            const IV: Option<&'static [u8]> = Some(&[ #(#iv , )* ]);
            AESString::new(&BYTES, &KEY, IV)
        }
    );

    TokenStream::from(q)
}
#[cfg(test)]
mod tests {
    use crate::aes::include_aes_string_impl;
    use quote::quote;

    #[test]
    fn aes_comparison_operators() {
        let key: Vec<u8> = (1..=32).collect();
        let iv: Vec<u8> = (1..=16).collect();
        let q = quote! { "Test String", [ #(#key , )* ], [ #(#iv , )* ] };
        let result = include_aes_string_impl(q.into()).to_string();
        assert_eq!(result, "{ const BYTES : [u8 ; 16usize] = [10u8 , 129u8 , 250u8 , 66u8 , 74u8 , 78u8 , 174u8 , 203u8 , 19u8 , 171u8 , 136u8 , 241u8 , 166u8 , 188u8 , 95u8 , 70u8 ,] ; const KEY : [u8 ; 16usize] = [1u8 , 2u8 , 3u8 , 4u8 , 5u8 , 6u8 , 7u8 , 8u8 , 9u8 , 10u8 , 11u8 , 12u8 , 13u8 , 14u8 , 15u8 , 16u8 , 17u8 , 18u8 , 19u8 , 20u8 , 21u8 , 22u8 , 23u8 , 24u8 , 25u8 , 26u8 , 27u8 , 28u8 , 29u8 , 30u8 , 31u8 , 32u8 ,] ; AESString :: new (& BYTES , & KEY) }", "Could not compare AES_STRING and &str");
    }
}
