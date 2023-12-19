    use crate::resource_builder::ResourceBuilder;
    #[test]
    #[should_panic]
    fn test() {
        let res = ResourceBuilder::new("placeholder".to_string())
            .add_xor_strs(&["string1", "string2"])
            .add_xor_strs(&["string3"])
            .add_xor_strings(&["string4".to_string(), "string5".to_string()]);

        res.build();
    }