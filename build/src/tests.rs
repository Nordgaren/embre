use crate::resource_builder::ResourceBuilder;
#[test]
#[should_panic]
fn duplicate_panic() {
    let dir = std::env::current_dir().unwrap();
    let res = ResourceBuilder::new(dir.to_str().unwrap().to_string())
        .add_xor_strs(&["string1", "string2"])
        .add_xor_strs(&["string3"])
        .add_xor_strs(&["duplicate string"])
        .add_xor_strs(&["duplicate string"])
        .add_xor_strings(&["string4".to_string(), "string5".to_string()]);
    res.build();
}
