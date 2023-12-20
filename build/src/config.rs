use std::ops::RangeInclusive;

pub struct BuildConfig {
    pub resource_id: u32,
    pub resource_name: String,
    pub pad_range: RangeInclusive<usize>,
}

impl BuildConfig {
    pub fn new(
        resource_id: u32,
        resource_name: String,
        pad_range: RangeInclusive<usize>,
    ) -> BuildConfig {
        BuildConfig {
            resource_id,
            resource_name,
            pad_range,
        }
    }
}

pub const DEFAULT_RESOURCE_ID: u32 = 100;
pub const PAD_RANGE_START: usize = 0;
pub const PAD_RANGE_END: usize = 0x100;
pub const DEFAULT_PAD_RANGE: RangeInclusive<usize> = PAD_RANGE_START..=PAD_RANGE_END;
impl Default for BuildConfig {
    fn default() -> BuildConfig {
        BuildConfig::new(
            DEFAULT_RESOURCE_ID,
            DEFAULT_RESOURCE_NAME.to_string(),
            DEFAULT_PAD_RANGE,
        )
    }
}

pub const DEFAULT_RESOURCE_NAME: &'static str = "resource.bin";
