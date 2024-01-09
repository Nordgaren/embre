#[cfg(test)]
mod tests {
    use crate::consts::*;
    use embre::embedded_resource::EmbeddedAES;

    #[test]
    fn get_aes_string() {
        let aes_string = RESOURCE_INFO.get_str(MY_AES_STRING);
        assert!("My AES string!" == aes_string)
    }
    #[test]
    fn get_named_aes_string() {
        let aes_string = RESOURCE_INFO.get_str(NAMED_AES);
        assert!("My named AES string" == aes_string)
    }
}
