use std::sync::LazyLock;

pub struct FontConfig {
    pub is_little_endian: bool,
}

pub static FONT_CONFIG: LazyLock<FontConfig> = LazyLock::new(|| FontConfig {
    is_little_endian: cfg!(target_endian = "little"),
});