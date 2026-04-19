use serde::{Deserialize, Serialize};

/// Represents a complete set of mod requirements for a Minecraft installation
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModRequirements {
    /// The Minecraft version these mods are for (e.g., "1.20.1")
    pub minecraft_version: String,
    /// List of required and optional mods
    pub mods: Vec<Mod>,
}

/// Represents a single mod with its requirements
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mod {
    /// Mod name (human-readable)
    pub name: String,
    /// Mod filename (.jar)
    pub filename: String,
    /// Specific version of the mod
    pub version: String,
    /// Whether this mod is required (true) or optional (false)
    #[serde(default)]
    pub required: bool,
    /// Optional download URL for the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ModRequirements {
    /// Creates a new ModRequirements instance
    pub fn new(minecraft_version: String) -> Self {
        Self {
            minecraft_version,
            mods: Vec::new(),
        }
    }

    /// Adds a mod to the requirements
    pub fn add_mod(&mut self, mod_entry: Mod) {
        self.mods.push(mod_entry);
    }

    /// Returns the number of required mods
    pub fn required_count(&self) -> usize {
        self.mods.iter().filter(|m| m.required).count()
    }

    /// Returns the number of optional mods
    pub fn optional_count(&self) -> usize {
        self.mods.iter().filter(|m| !m.required).count()
    }
}

impl Mod {
    /// Creates a new required mod
    pub fn required(name: String, filename: String, version: String) -> Self {
        Self {
            name,
            filename,
            version,
            required: true,
            url: None,
        }
    }

    /// Creates a new optional mod
    pub fn optional(name: String, filename: String, version: String) -> Self {
        Self {
            name,
            filename,
            version,
            required: false,
            url: None,
        }
    }

    /// Sets the download URL for this mod
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}
