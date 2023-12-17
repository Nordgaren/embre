use crate::config::BuildConfig;
use crate::resource::aes_resource::AESResource;
use crate::resource::plaintext_resource::PlaintextResource;
use crate::resource::xor_resource::XORResource;
use crate::resource::GetResourceName;
use crate::util::generate_random_bytes;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::{env, fs};
use winresource::WindowsResource;

pub struct ResourceBuilder {
    out_dir: String,
    config: BuildConfig,
    resource_bytes: Vec<u8>,
    plaintext_resources: Vec<PlaintextResource>,
    aes_resources: Vec<AESResource>,
    pub(super) xor_resources: Vec<XORResource>,
}

impl ResourceBuilder {
    pub fn new(out_dir: String) -> Self {
        ResourceBuilder {
            out_dir,
            config: Default::default(),
            resource_bytes: vec![],
            plaintext_resources: vec![],
            aes_resources: vec![],
            xor_resources: vec![],
        }
    }
    pub fn add_config(mut self, config: BuildConfig) -> Self {
        self.config = config;
        self
    }
    pub fn add_strings_xor(self, strings: &[String]) -> Self {
        let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        self.add_strs_xor(strs.as_slice())
    }
    pub fn add_strs_xor(mut self, strs: &[&str]) -> Self {
        self.xor_resources.extend(strs.iter().map(|string_name| {
            XORResource::from_str(string_name, generate_random_bytes(string_name.len()))
        }));

        self
    }
    pub fn add_str_xor(mut self, new_string: &str) -> Self {
        self.xor_resources.push(XORResource::from_str(
            new_string,
            generate_random_bytes(new_string.len()),
        ));
        self
    }
    pub fn build_resource_binary(mut self) -> Self {
        // Put these functions into a vector we can then pop functions out of, to randomize position of resources.
        let mut resources = vec![];

        for resource in self.aes_resources.iter_mut() {
            resources.push(&mut resource.encrypted_resource);
            resources.push(&mut resource.key);
            if let Some(iv) = resource.iv.as_mut() {
                resources.push(iv)
            }
        }

        for resource in self.xor_resources.iter_mut() {
            resources.push(&mut resource.encrypted_resource);
            resources.push(&mut resource.key);
        }

        for resource in self.plaintext_resources.iter_mut() {
            resources.push(&mut resource.resource);
        }

        resources.shuffle(&mut rand::thread_rng());
        for res in resources {
            let bytes =
                generate_random_bytes(rand::thread_rng().gen_range(self.config.pad_range.clone()));
            self.resource_bytes.extend(bytes);

            res.offset = self.resource_bytes.len();
            self.resource_bytes.extend(&res.bytes);
        }

        let end_pad =
            generate_random_bytes(rand::thread_rng().gen_range(self.config.pad_range.clone()));
        self.resource_bytes.extend(end_pad);

        fs::write(
            format!("{}/{}", self.out_dir, "resource.bin"),
            &self.resource_bytes,
        )
        .expect("Could not write payload file.");

        self
    }
    pub fn build_consts_file(self) -> Self {
        let mut consts = vec!["#![allow(unused)]".to_string()];

        consts.push(format!(
            "pub const {}: u32 = {:#X};",
            "RESOURCE_ID", self.config.resource_id
        ));

        consts.push(format!("pub const {}: usize = {:#X};", "RT_RCDATA", 10));

        self.xor_resources.iter().for_each(|string| {
            consts.push(format!(
                "pub const {}_POS: usize = {:#X};",
                string.resource_name, string.encrypted_resource.offset
            ));
            consts.push(format!(
                "pub const {}_LEN: usize = {:#X};",
                string.resource_name,
                string.encrypted_resource.bytes.len()
            ));
            consts.push(format!(
                "pub const {}_KEY: usize = {:#X};",
                string.resource_name, string.key.offset
            ));
        });

        fs::write("src/consts.rs", consts.join("\n")).expect("Could not write consts file.");

        self
    }
    pub fn build_resource_headers(self) -> Self {
        fs::write(
            format!("{}/resources.h", self.out_dir),
            format!("#define PAYLOAD_ID {}\n", self.config.resource_id),
        )
        .expect("Could not write resources.h file.");

        fs::write(
            format!("{}/resources.rc", self.out_dir),
            format!(
                "#include \"resources.h\"\nPAYLOAD_ID RCDATA {}\n",
                "resource.bin"
            ),
        )
        .expect("Could not write resources.rc file.");

        self
    }
    pub fn build(self) {
        self.check_duplicate_entries();

        // Build the resource file
        let s = self
            .build_resource_binary()
            // Builds src/consts.rs for use in the actual application
            .build_consts_file()
            // Build the resource header files for embedding.
            .build_resource_headers();

        if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
            WindowsResource::new()
                .set_resource_file(&format!("{}/resources.rc", s.out_dir))
                .compile()
                .expect("Could not compile pe resource.");
        }
    }
    pub(super) fn get_resource_names(&self) -> Vec<&String> {
        self.xor_resources
            .iter()
            .map(|r| r.get_resource_name())
            .chain(self.aes_resources.iter().map(|r| r.get_resource_name()))
            .chain(
                self.plaintext_resources
                    .iter()
                    .map(|r| r.get_resource_name()),
            )
            .collect()
    }
    fn check_duplicate_entries(&self) {
        let mut names: Vec<_> = self.get_resource_names();
        let orig_len = names.len();
        names.sort();
        names.dedup();
        if names.len() != orig_len {
            panic!("Duplicate names detected")
        }
    }
}
