use std::fmt;

/// Errors that can occur during validation of mod requirements
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// A mod has an empty ID at the given index
    EmptyId(usize),
    /// A mod has an empty version at the given index
    EmptyVersion(usize),
    /// The Minecraft version is empty or invalid
    InvalidMinecraftVersion,
    /// The mod list is empty
    EmptyModList,
    /// A mod has an invalid URL at the given index
    InvalidUrl(usize, String),
    /// Duplicate mod IDs found
    DuplicateModId(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::EmptyId(idx) => {
                write!(f, "Mod at index {} has empty id", idx)
            }
            ValidationError::EmptyVersion(idx) => {
                write!(f, "Mod at index {} has empty version", idx)
            }
            ValidationError::InvalidMinecraftVersion => {
                write!(f, "Minecraft version is empty or invalid")
            }
            ValidationError::EmptyModList => {
                write!(f, "Mod list is empty")
            }
            ValidationError::InvalidUrl(idx, url) => {
                write!(f, "Mod at index {} has invalid URL: {}", idx, url)
            }
            ValidationError::DuplicateModId(id) => {
                write!(f, "Duplicate mod id found: {}", id)
            }
        }
    }
}

impl std::error::Error for ValidationError {}