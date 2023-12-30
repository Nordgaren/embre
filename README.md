# EmbRe - Embedded Resources API for Rust  
If GIF can be pronounced "JIF", then my jift to the world is that this crate is pronounced "Ember".
A crate for encrypting, embedding and comparing encrypted resources to non encrypted resources, in Rust.

## Macro
You can include encrypted bytes or strings with the macros that this crate offers.

```rust
// You can include files as encrypted bytes with a full or relative path.  
// You can compare them with other plaintext buffers as if they were plaintext byte slices. 
let xor_bytes = include_xor_bytes!("P:/ath/to/file.bin");
let aes_bytes = include_aes_bytes!("relative/path/file.bin");
assert!(xor_bytes == include_bytes!("P:/ath/to/file.bin"));
assert!(aes_bytes == include_bytes!("relative/path/file.bin"));

// You can also include xor encrypted strings using the raw string, or a path to the file.
// These string types can be directly compared
let xor_string = include_xor_string!("My String");
let aes_string = include_aes_string!("./string.file");
assert!(xor_string == "test string");
assert!(aes_string == /* contents of './string.file' */);
```
XOR data/strings can be created as consts/statics. I hope to be able to do the same with AES, soon.

## Build
You can embed strings in a PE resource. Currently this just automatically calls the `winresource` crate, but in the future 
I would like to add the ability for the user to build and embed the resource, however they would like, by just passing back 
a vector.
```rust
use embre_build::resource_builder::ResourceBuilder;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    ResourceBuilder::new(out_dir)
        // named strings allow you to determine the name of the constant for your strings
        .add_named_xor_str("named xor", "My name XOR string") // NAMED_XOR_POS NAMED_XOR_KEY NAMED_XOR_LEN
        // no named variants will use the full string as the constant name
        .add_xor_str("My xor string") // MY_XOR_STRING_POS MY_XOR_STRING_KEY MY_XOR_STRING_LEN
        // same goes for aes encrypted strings
        .add_named_aes_str("named aes", "My named AES string") // NAMED_AES_POS NAMED_AES_KEY NAMED_AES_IV NAMED_AES_LEN
        // spaces are replaced with '_' for all constant names, and all symbols are removed.
        .add_aes_str("My AES string!") // MY_AES_STRING_POS MY_AES_STRING_KEY MY_AES_STRING_IV MY_AES_STRING_LEN
        .build();
}
```
You can then use the default PEResource struct, using the feature `DefaultPEResource`. This will load the PE resource via 
the Windows API. You can also implement your own get_resource for PEs, by implementing the `EmbeddedResource` trait. This 
trait is still in development.

```rust
let pe = PEResource::new(RT_RCDATA, RESOURCE_ID);
let name_xor_string = pe.get_xor_string(NAMED_XOR_POS, NAME_XOR_KEY, NAME_XOR_LEN);
let xor_string = pe.get_xor_string(MY_XOR_POS, MY_XOR_KEY, MY_XOR_LEN);
let name_aes_string = pe.get_xor_string(NAMED_AES_POS, NAMED_AES_KEY, NAMED_AES_LEN);
let aes_string = pe.get_xor_string(MY_AES_POS, MY_AES_KEY, MY_AES_LEN);
```
I am not sure if I want to make a dedicated structure for these, yet, or not.

## Sub crates  
You should only have to import the main crate. The sub crates are for development/organizational purposes, only.  

`_test` - Testing crate for the macro crate. Do not import.  
`build` - This crate holds the build utilities for creating a binary blob to be embedded somewhere, like the PE Resource 
section of a PE.  
`core`  - The implementation for the macros in the macro crate.  
`crypt` - A crypt crate that holds all the encryption implementations for openssl. Might move this to the main crate.  
`macro` - Macros for encrypting and embedding resources directly in your codebase, instead of using the build method.  
`ember` - The main crate. This is where the abstractions for handling and comparing the encrypted resources, are.  

## Goals  
> Right now I want to support as many AES encryption methods as I can, and make macros for each, maybe.   
> I would like to figure out a way to get the AES resource functions to be const compile time methods. Add embedded resource 
  handling, maybe.  
> I also want the user to be able to define their own implementation for the encryption/decryption process. For example 
  the AESCrypter trait can be implemented for a user defined type, and then used in the `AESResource<'a, T, C: AESCrypter>` 
  type, and the program will use the implementation the user defined. This way the user could handle things differently, 
  or use a different encryption library, for whatever reason.  

## Thank You
[RoseHasANose](https://github.com/largenumberhere) - Thank you for the LitBytes parser code for the import macros, and helping me figure out how to parse 
arguments the way I want them!  

## Todo
> Possibly move crypt crate into main crate. Definitely make vendored openssl optional. 
