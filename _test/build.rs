use embre_build::resource::aes_resource::AESResource;
use embre_build::resource_builder::ResourceBuilder;

fn main() {
    ResourceBuilder::default()
        // named strings allow you to determine the name of the constant for your strings
        .add_xor_resource(("named xor", "My named XOR string")) // NAMED_XOR_POS NAMED_XOR_KEY NAMED_XOR_LEN
        // no named variants will use the full string as the constant name
        .add_xor_resource("My XOR string") // MY_XOR_STRING_POS MY_XOR_STRING_KEY MY_XOR_STRING_LEN
        // same goes for aes encrypted strings
        // spaces are replaced with '_' for all constant names, and all symbols are removed.
        .add_aes_resource(AESResource::named_str("named aes", "My named AES string")) // NAMED_AES_POS NAMED_AES_KEY NAMED_AES_IV NAMED_AES_LEN
        .add_aes_resource("My AES string!") // MY_AES_STRING_POS MY_AES_STRING_KEY MY_AES_STRING_IV MY_AES_STRING_LEN
        .build();
}
