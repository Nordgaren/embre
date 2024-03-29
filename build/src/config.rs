use std::ops::RangeInclusive;
/// A BuildConfig for ResourceBuilder that has the category_id, resource_id and resource_name, as well as the pad range
/// of bytes between resources in the binary blob.
///
/// # Fields
///
/// * `category_id`: `u32` - Category ID of the resource in a PE file.
/// * `resource_id`: `u32` - Resource ID of the resource in a PE file.
/// * `resource_name`: `String` - Name of the resource to be written to disk for embedding.
/// * `pad_range`: `RangeInclusive<usize>` - Range of randomly generated bytes between resources.
///
/// # Examples
///
/// ```rust
/// # use embre_build::config::BuildConfig;
/// // These example values also happen to be the default values for BuildConfig::default();
/// let config = BuildConfig::new(10, 100, "resource.bin".to_string(), 0..=0x100);
/// ```
pub struct BuildConfig {
    pub category_id: u32,
    pub resource_id: u32,
    pub resource_name: String,
    pub pad_range: RangeInclusive<usize>,
}

impl BuildConfig {
    /// Returns a new BuildConfig with the provided values.
    ///
    /// # Arguments
    ///
    /// * `category_id`: `u32` - Category ID of the resource in a PE file.
    /// * `resource_id`: `u32` - Resource ID of the resource in a PE file.
    /// * `resource_name`: `String` - Name of the resource to be written to disk for embedding.
    /// * `pad_range`: `RangeInclusive<usize>` - Range of randomly generated bytes between resources.
    pub fn new(
        category_id: u32,
        resource_id: u32,
        resource_name: String,
        pad_range: RangeInclusive<usize>,
    ) -> BuildConfig {
        BuildConfig {
            category_id,
            resource_id,
            resource_name,
            pad_range,
        }
    }
}

pub const PAD_RANGE_START: usize = 0;
pub const PAD_RANGE_END: usize = 0x100;
pub const DEFAULT_RESOURCE_ID: u32 = 100;
pub const DEFAULT_CATEGORY_ID: u32 = 10; // RT_RCDATA
pub const DEFAULT_RESOURCE_NAME: &str = "resource.bin";
pub const DEFAULT_PAD_RANGE: RangeInclusive<usize> = PAD_RANGE_START..=PAD_RANGE_END;
impl Default for BuildConfig {
    /// Provides the default BuildConfig.
    ///
    /// # Default Values
    ///
    /// * `category_id`: 10
    /// * `resource_id`: 100
    /// * `resource_name`: "resource.bin"
    /// * `pad_range`: 0..=0x100
    ///
    /// returns: BuildConfig
    fn default() -> BuildConfig {
        BuildConfig::new(
            DEFAULT_CATEGORY_ID,
            DEFAULT_RESOURCE_ID,
            DEFAULT_RESOURCE_NAME.to_string(),
            DEFAULT_PAD_RANGE,
        )
    }
}
