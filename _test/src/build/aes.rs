#[cfg(test)]
mod tests {
    use embre::embedded_resource::EmbeddedAES;
    use crate::consts::*;

    #[test]
    fn get_aes_string() {
        let aes_string = RESOURCE_INFO.get_str(MY_AES_STRING);
        assert!("My AES string!" == aes_string)
    }
    #[test]
    fn get_named_aes_string() {
        let aes_string = RESOURCE_INFO.get_str(NAMED_AES);
        println!("{}", aes_string.to_plaintext_string().unwrap());
        assert!("My named AES string" == aes_string)
    }
}