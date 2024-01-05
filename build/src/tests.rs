use crate::resource_builder::ResourceBuilder;
#[test]
#[should_panic]
fn duplicate_panic() {
    ResourceBuilder::default()
        .add_xor_resource("duplicate string")
        .add_xor_resource("duplicate string")
        .build();
}
