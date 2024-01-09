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

pub fn xor_bytes_in_place(bytes: &mut [u8], key: &[u8]) {
    if bytes.len() != key.len() {
        panic!("Key and source len differ.")
    }

    for i in 0..bytes.len() {
        bytes[i] ^= key[i];
    }
}
