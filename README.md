# EmbRe - Embedded Resources API for Rust  
If GIF can be pronounced "JIF", then my jift to the world is that this crate is pronounced "Ember".
A crate for encrypting, embedding and comparing encrypted resources to non encrypted resources, in Rust.


## Sub crates  
`_test` - Testing crate for the macro crate. Do not import.  
`build` - This crate holds the build utilities for creating a binary blob to be embedded somewhere, like the PE Resource section of a PE.  
`core`  - The implementation for the macros in the macro crate.  
`crypt` - A crypt crate that holds all the encryption implementations for openssl. Might move this to the main crate.  
`macro` - Macros for encrypting and embedding resources directly in your codebase, instead of using the build method.  
`ember` - The main crate. This is where the abstractions for handling and comparing the encrypted resources, are.  

## Goals  
> Right now I want to support as many AES encryption methods as I can, and make macros for each, maybe.   
> I would like to figure out a way to get the AES resource functions to be const compile time methods. Add embedded resource handling, maybe.  
> I also want the user to be able to define their own implementation for the encryption/decryption process. For example the AESCrypter trait can be implemented for a user defined type, and then used in the `AESResource<'a, T, C: AESCrypter>`type, and the program will use the implementation the user defined. This way the user could handle things differently, or use a different encryption library, for whatever reason.  

## Thank You
[RoseHasANose](https://github.com/largenumberhere) - Thank you for the LitBytes parser code for the import macros, and helping me figure out how to parse arguments the way I want them!  