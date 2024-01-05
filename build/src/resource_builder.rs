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
/// A Resource builder for building a binary blob for embedding into a PE (or soon, an ELF). This builder also provides
/// offsets to the data in the binary blob, and can also do the embedding process into a PE resource, for you.  Resources
/// are inserted randomly into the binary blob on build, with random padding between each resource, determined by the config.
///
/// The ResourceBuilder also writes a consts.rs file with constants that can be used with the EmbeddedResource trait to
/// retrieve the users data.
///
/// All resource names must be unique, and are checked during `.build()`.
///
/// # Fields
///
/// * `out_dir`: String - The directory where all generated files are written to.
/// * `config`: BuildConfig - Config type that allows the user to specify the category id, resource id, name, and pad range of the
/// resource builder.
///
/// # Examples
///
/// ```rust
/// # use embre_build::config::BuildConfig;
/// # use embre_build::resource_builder::ResourceBuilder;
/// let config = BuildConfig::new(10, 100, "my_resource.bin".to_string(), 0..=0x100);
/// ResourceBuilder::new("P:/ath/to/out_dir".to_string(), config)
///         .add_xor_resource("My String")
///         .build()
/// ```
pub struct ResourceBuilder {
    out_dir: String,
    config: BuildConfig,
    resource_bytes: Vec<u8>,
    plaintext_resources: Vec<PlaintextResource>,
    aes_resources: Vec<AESResource>,
    pub(super) xor_resources: Vec<XORResource>,
}

impl Default for ResourceBuilder {
    /// Returns a new Resource builder with the 'OUT_DIR' environment variable value as the out directory and the default
    /// BuildConfig settings. You can supply a build config after, with the builder methods.
    ///
    /// returns: ResourceBuilder
    ///
    /// # Default Values
    ///
    /// * `out_dir`: env::var("OUT_DIR")
    /// * `config`: Default::default()
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embre_build::config::BuildConfig;
    /// # use embre_build::resource_builder::ResourceBuilder;
    /// let config = BuildConfig::new(10, 100, "my_resource.bin".to_string(), 0..=0x100);
    /// ResourceBuilder::default()
    ///         .add_xor_resource("My String")
    ///         .with_config(config)
    ///         .build()
    /// ```
    fn default() -> Self {
        ResourceBuilder::new(
            env::var("OUT_DIR").expect("Could not get environment variable OUT_DIR"),
            Default::default(),
        )
    }
}

impl ResourceBuilder {
    /// Returns a new Resource builder with the out_dir and config provided. `ResourceBuilder::default()` is almost always
    /// better, but this function is provided in-case the user decides to use a different out directory.
    pub fn new(out_dir: String, config: BuildConfig) -> Self {
        ResourceBuilder {
            out_dir,
            config,
            resource_bytes: vec![],
            plaintext_resources: vec![],
            aes_resources: vec![],
            xor_resources: vec![],
        }
    }
    /// Changed the config of the current resource builder to the supplied config.
    ///
    /// returns: ResourceBuilder
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embre_build::config::BuildConfig;
    /// # use embre_build::resource_builder::ResourceBuilder;
    /// let config = BuildConfig::new(10, 100, "my_resource.bin".to_string(), 0..=0x100);
    /// ResourceBuilder::default()
    ///         .with_config(config);
    /// ```
    pub fn with_config(mut self, config: BuildConfig) -> Self {
        self.config = config;
        self
    }
    /// Add multiple Strings at a time to be xor encrypted. All strings will be auto-named for lookup constants.
    pub fn add_xor_strings(self, strings: &[String]) -> Self {
        let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        self.add_xor_strs(strs.as_slice())
    }
    /// Add multiple &strs at a time to be xor encrypted. All strings will be auto-named for lookup constants.
    pub fn add_xor_strs(mut self, strs: &[&str]) -> Self {
        self.xor_resources.extend(
            strs.iter()
                .map(|string_name| XORResource::from_str(string_name)),
        );
        self
    }
    /// Adds a resource to the resource builder for embedding in an executable within a binary blob as xor encrypted data.
    /// Strings will be named after themselves, with the util::make_const_name function, which will remove illegal characters
    /// and turn letters uppercase. You can provide a name, to not leave the naming up to the algorithm, as well as provide
    /// a stable const name for lookup in the resulting embedded blob.
    ///
    /// # Arguments
    ///
    /// * `resource`: impl Into\<XORResource\> - Anything that implements Into<XORResource>. This can be an XORResource, itself, or a String, &str,
    /// a tuple of (&str, &str) or tuple of (&str, &[u8]).
    ///
    /// returns: ResourceBuilder
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embre_build::resource::xor_resource::XORResource;
    /// # use embre_build::resource_builder::ResourceBuilder;
    /// # use embre_build::util;
    ///
    /// # let some_var = 64;
    /// ResourceBuilder::default()
    ///         .add_xor_resource("My String")
    ///         .add_xor_resource(("string 2", "MyString2"))
    ///         .add_xor_resource(("bytes", &[10, 11, 12, 13, 14, 15]))
    ///         .add_xor_resource(format!("Some formatted string {}", some_var))
    ///         .add_xor_resource(XORResource::new("resource name", "resource string".as_bytes(), util::generate_random_bytes("resource string".len())))
    ///         .build();
    /// ```
    pub fn add_xor_resource(mut self, resource: impl Into<XORResource>) -> Self {
        self.xor_resources.push(resource.into());
        self
    }
    /// Add multiple Strings at a time to be aes encrypted. All strings will be auto-named for lookup constants.
    pub fn add_aes_strings(self, strings: &[String]) -> Self {
        let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        self.add_xor_strs(strs.as_slice())
    }
    /// Add multiple &strs at a time to be aes encrypted. All strings will be auto-named for lookup constants.
    pub fn add_aes_strs(mut self, strs: &[&str]) -> Self {
        self.aes_resources.extend(
            strs.iter()
                .map(|string_name| AESResource::from_str(string_name, None, None)),
        );

        self
    }
    /// Adds a resource to the resource builder for embedding in an executable within a binary blob as aes encrypted data.
    /// Strings will be named after themselves, with the util::make_const_name function, which will remove illegal characters
    /// and turn letters uppercase. You can provide a name, to not leave the naming up to the algorithm, as well as provide
    /// a stable const name for lookup in the resulting embedded blob.
    ///
    /// # Arguments
    ///
    /// * `resource`: impl Into\<AESResource\> - Anything that implements Into<AESResource>. This can be an AESResource, itself, or a String, &str,
    /// a tuple of (&str, &str) or tuple of (&str, &[u8]).
    ///
    /// returns: ResourceBuilder
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embre_build::resource::aes_resource::AESResource;
    /// # use embre_build::resource_builder::ResourceBuilder;
    /// # use embre_build::util;
    ///
    /// # let some_var = 64;
    /// # let key_len = 32;
    /// ResourceBuilder::default()
    ///         .add_aes_resource("My String")
    ///         .add_aes_resource(("string 2", "MyString2"))
    ///         .add_aes_resource(("bytes", &[10, 11, 12, 13, 14, 15]))
    ///         .add_aes_resource(format!("Some formatted string {}", some_var))
    ///         .add_aes_resource(AESResource::new("resource name", "resource string".as_bytes(), Some(util::generate_random_bytes(key_len)), None))
    ///         .build();
    /// ```
    pub fn add_aes_str(mut self, resource: impl Into<AESResource>) -> Self {
        self.aes_resources
            .push(resource.into());
        self
    }
    pub fn build_resource_binary(mut self) -> Self {
        // Put these functions into a vector we can then pop functions out of, to randomize position of resources.
        let mut resources = vec![];

        for resource in self.aes_resources.iter_mut() {
            resources.push(&mut resource.encrypted_resource);
            resources.push(&mut resource.key);
            resources.push(&mut resource.iv)
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
        let mut consts = vec![];

        consts.push(format!(
            "pub const {}: embre::embedded_resource::PEResource = embre::embedded_resource::PEResource::new({:#X}, {:#X});",
            "RESOURCE_INFO", self.config.category_id, self.config.resource_id
        ));

        self.xor_resources.iter().for_each(|string| {
            consts.push(format!(
                "pub const {}: embre::embedded_resource::XOROffsets = embre::embedded_resource::XOROffsets::new({:#X}, {:#X}, {:#X});",
                string.resource_name, string.encrypted_resource.offset, string.key.offset, string.encrypted_resource.bytes.len()
            ));
        });

        self.aes_resources.iter().for_each(|string| {
            consts.push(format!(
                "pub const {}: embre::embedded_resource::XOROffsets = embre::embedded_resource::XOROffsets::new({:#X}, {:#X}, Some({:#X}), {:#X});",
                string.resource_name, string.encrypted_resource.offset, string.key.offset, string.iv.offset, string.encrypted_resource.bytes.len()
            ));
        });

        fs::write(format!("{}/consts.rs", self.out_dir), consts.join("\n"))
            .expect("Could not write consts file.");

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

impl From<String> for XORResource {
    fn from(string: String) -> Self {
        XORResource::from(&string[..])
    }
}
impl From<&str> for XORResource {
    fn from(string: &str) -> Self {
        XORResource::from_str(string)
    }
}
impl From<(&str, &str)> for XORResource {
    fn from((name, string): (&str, &str)) -> Self {
        XORResource::named_str(name, string)
    }
}
impl From<(&str, &[u8])> for XORResource {
    fn from((name, data): (&str, &[u8])) -> Self {
        XORResource::named(name, data)
    }
}
impl From<String> for AESResource {
    fn from(string: String) -> Self {
        AESResource::from(&string[..])
    }
}
impl From<&str> for AESResource {
    fn from(string: &str) -> Self {
        AESResource::from_str(string, None, None)
    }
}
impl From<(&str, &str)> for AESResource {
    fn from((name, string): (&str, &str)) -> Self {
        AESResource::named_str(name, string)
    }
}
impl From<(&str, &[u8])> for AESResource {
    fn from((name, data): (&str, &[u8])) -> Self {
        AESResource::named(name, data)
    }
}
