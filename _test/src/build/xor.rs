
#[cfg(test)]
mod tests {
    use embre::embedded_resource::EmbeddedXOR;
    use crate::consts::*;

    #[test]
    fn get_xor_string() {
        let xor_string = RESOURCE_INFO.get_str(MY_XOR_STRING);
        assert_eq!("My XOR string", xor_string)
    }
    #[test]
    fn get_named_xor_string() {
        let xor_string = RESOURCE_INFO.get_str(NAMED_XOR);
        assert_eq!("My named XOR string", xor_string)
    }
}