    use crate::resource::xor_resource::XORResource;
    use crate::resource_builder::ResourceBuilder;
    #[test]
    #[should_panic]
    fn test() {
        let res = ResourceBuilder::new("placeholder".to_string())
            .add_strs_xor(&["kek", "lol"])
            .add_strs_xor(&["keklol"])
            .add_strings_xor(&["lolokek".to_string(), "kekekeklol".to_string()])
            .add_strings_xor(&["lolokek".to_string(), "kekekeklol".to_string()]);

        res.build();
    }