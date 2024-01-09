pub fn xor_bytes(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    if bytes.len() != key.len() {
        panic!("Key and source len differ.")
    }

    let mut out = bytes.to_vec();

    for i in 0..bytes.len() {
        out[i] ^= key[i];
    }

    out
}