# EmbRe - Embedded Resources API for Rust  
[GitHub](https://github.com/Nordgaren/embre)  
If GIF can be pronounced "JIF", then my jift to the world is that this crate is pronounced "Ember".
A crate for encrypting, embedding and comparing encrypted resources to non encrypted resources, in Rust.

## Known Issue
> For some reason, openssl has to be vendored. If not, the embre crate will not build, because it will be unable to find 
the "embre-macro" crate, for some reason. I am not sure why. I do want to fix it, asap, though! Update: May or may not have
fixed itself. IDK. :(

> AES portion of crate still needs a lot of testing.  

## Macro Embedding
You can include encrypted bytes or strings with the macros that this crate offers.

```rust
// You can include files as encrypted bytes with a full or relative path.  
// You can compare them with other plaintext buffers as if they were plaintext byte slices. 
const XOR_BYTES: XORBytes = include_xor_bytes!("P:/ath/to/file.bin");
fn bytes() {
    let aes_bytes = include_aes_bytes!("relative/path/file.bin");
    assert!(XOR_BYTES == include_bytes!("P:/ath/to/file.bin"));
    assert!(aes_bytes == include_bytes!("relative/path/file.bin"));
}
// You can also include xor encrypted strings using the raw string, or a path to the file.
// These string types can be directly compared
const XOR_STRING: XORString = include_xor_str!("My String");
fn strings() {
    let aes_string = include_aes_str!("./string.file");
    assert!(XOR_STRING == "test string");
    assert!(aes_string == include_str!("./string.file"));
}
```
XOR data/strings can be created as consts/statics. I hope to be able to do the same with AES, soon.

## Build Script Embedding
You can embed strings in a PE resource. Currently this just automatically calls the `winresource` crate, but in the future 
I would like to add the ability for the user to build and embed the resource, however they would like, by just passing back 
a vector.
```rust
// build.rs

use embre_build::resource_builder::ResourceBuilder;

fn main() {
    ResourceBuilder::default()
        // named strings allow you to determine the name of the constant for your strings
        .add_xor_resource(("named xor", "My named XOR string")) // NAMED_XOR_POS NAMED_XOR_KEY NAMED_XOR_LEN
        // no named variants will use the full string as the constant name
        .add_xor_resource("My xor string") // MY_XOR_STRING_POS MY_XOR_STRING_KEY MY_XOR_STRING_LEN
        // same goes for aes encrypted strings
        .add_aes_resource(AESResource::named_str("named aes", "My named AES string")) // NAMED_AES_POS NAMED_AES_KEY NAMED_AES_IV NAMED_AES_LEN
        // spaces are replaced with '_' for all constant names, and all symbols are removed.
        .add_aes_resource("My AES string!") // MY_AES_STRING_POS MY_AES_STRING_KEY MY_AES_STRING_IV MY_AES_STRING_LEN
        .build();
}
```
You can then include the generated consts file using the `include!` macro, and use the default PEResource struct to get 
the embedded resources, using the feature `DefaultPEResource`. This will load the PE resource via the Windows API. You 
can also implement your own PEResource struct and get_resource implementation for PEs, by implementing the `EmbeddedResource` 
trait. This trait is still in development, and signatures may change in future updates.

Currently the traits to get the resource have the same name, in case your build has any type of symbols. get_str and get_data.
If you pass in AESOffsets, you get AESData or AESString. If you pass in XOROffsets, you will get the XORData or XORString you
requested. 

In the future I need to change these traits so that the user can get their custom AESResource or XORResource impls.  

```rust
// Include the generated consts file that is in the out dir.  
include!(concat!(env!("OUT_DIR"), "/consts.rs"));

fn main() {
  let pe = RESOURCE_INFO;
  // Pass in XOROffsets to get XOR data and strings
  let name_xor_string = pe.get_str(NAMED_XOR);
  let xor_string = pe.get_data(MY_XOR);
  // Pass in AESOffets to get AES data and strings  
  let name_aes_string = pe.get_str(NAMED_AES);
  let aes_string = pe.get_data(MY_AES);
}
```
If you don't want to use `DefaultPEResource` implementation, you can implement your own and implement 
`From<embre::embedded_resource::PEResource>` to easily convert the const generated from the crate format to your format.  
```rust
use embre::embedded_resource::PEResource;

pub struct MyPEResource {
    category_id: u32,
    resource_id: u32,
}
impl MyPEResource { 
  // Any methods you might want to have, for your own struct.  
}

impl From<PEResource> for MyPEResource {
    fn from(value: PEResource) -> Self {
      MyPEResource {
            category_id: value.category_id,
            resource_id: value.resource_id,
        }
    }
}

impl EmbeddedResource for MyPEResource {
    fn get_resource(&self) -> Option<&'static [u8]> {
        unsafe {
            let addr = GetModuleHandleInternal(None);
            let pe = PE::from_address(addr).ok()?;

            pe.get_pe_resource(self.category_id, self.resource_id)
        }
    }
}

impl EmbeddedXOR for MyPEResource {}

impl EmbeddedAES for MyPEResource {}

fn main() {
  let pe = PEResource::from(RESOURCE_INFO);
  let name_xor_string = pe.get_str(NAMED_XOR);
  let xor_string = pe.get_str(MY_XOR);
  let name_aes_string = pe.get_str(NAMED_AES);
  let aes_string = pe.get_str(MY_AES);
}
```

This just uses the default implementation for `EmbeddedXOR` and `EmbeddedAES`, but you can also implement your own, as well.

## Sub crates  
You should only have to import the main crate. The sub crates are for development/organizational purposes, only.  

`_test` - Testing crate for the macro crate. Do not import.  
`build` - This crate holds the build utilities for creating a binary blob to be embedded somewhere, like the PE Resource 
section of a PE.  
`core`  - The implementation for the macros in the macro crate.  
`crypt` - A crypt crate that holds all the encryption implementations for openssl.   
`macro` - Macros for encrypting and embedding resources directly in your codebase, instead of using the build method.  
`utils` - Utils have now all been moved to a utils crate, for any shared functionality between sub-crates.  
`embre` - The main crate. This is where the abstractions for handling and comparing the encrypted resources, are.  

## Goals  
> Right now I want to support as many AES encryption methods as I can, and make macros for each, maybe.   
> I would like to figure out a way to get the AES resource functions to be const compile time methods. 
> I also want the user to be able to define their own implementation for the encryption/decryption process. For example 
  the AESCrypter trait can be implemented for a user defined type, and then used in the `AESResource<'a, T, C: AESCrypter>` 
  type, and the program will use the implementation the user defined. This way the user could handle things differently, 
  or use a different encryption library, for whatever reason.  

## Thank You
[RoseHasANose](https://github.com/largenumberhere) - Thank you for the LitBytes parser code for the import macros, and helping me figure out how to parse 
arguments the way I want them!  

## Todo
> Build a TODO list...  
