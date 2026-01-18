use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModRequirements {
    pub minecraft_version: String,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {
    pub id: String,
    pub version: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug)]
pub enum ValidationError {
    EmptyId(usize),
    EmptyVersion(usize),
    InvalidMinecraftVersion,
    EmptyModList,
    InvalidUrl(usize, String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::EmptyId(idx) => write!(f, "Mod at index {} has empty id", idx),
            ValidationError::EmptyVersion(idx) => write!(f, "Mod at index {} has empty version", idx),
            ValidationError::InvalidMinecraftVersion => write!(f, "Minecraft version is empty"),
            ValidationError::EmptyModList => write!(f, "Mod list is empty"),
            ValidationError::InvalidUrl(idx, url) => write!(f, "Mod at index {} has invalid URL: {}", idx, url),
        }
    }
}

fn validate_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn validate_requirements(reqs: &ModRequirements) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate Minecraft version
    if reqs.minecraft_version.trim().is_empty() {
        errors.push(ValidationError::InvalidMinecraftVersion);
    }

    // Validate mod list
    if reqs.mods.is_empty() {
        errors.push(ValidationError::EmptyModList);
    }

    // Validate each mod
    for (idx, mod_entry) in reqs.mods.iter().enumerate() {
        if mod_entry.id.trim().is_empty() {
            errors.push(ValidationError::EmptyId(idx));
        }
        
        if mod_entry.version.trim().is_empty() {
            errors.push(ValidationError::EmptyVersion(idx));
        }

        if let Some(url) = &mod_entry.url {
            if !validate_url(url) {
                errors.push(ValidationError::InvalidUrl(idx, url.clone()));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_requirements() {
        let yaml = r#"
minecraft_version: "1.20.1"
mods:
  - id: forge
    version: "47.1.0"
    required: true
  - id: jei
    version: "15.2.0.27"
    url: https://www.curseforge.com/test
    required: true
"#;
        let reqs: ModRequirements = serde_yaml::from_str(yaml).unwrap();
        assert!(validate_requirements(&reqs).is_ok());
    }

    #[test]
    fn test_empty_mod_id() {
        let yaml = r#"
minecraft_version: "1.20.1"
mods:
  - id: ""
    version: "1.0.0"
    required: true
"#;
        let reqs: ModRequirements = serde_yaml::from_str(yaml).unwrap();
        assert!(validate_requirements(&reqs).is_err());
    }

    #[test]
    fn test_invalid_url() {
        let yaml = r#"
minecraft_version: "1.20.1"
mods:
  - id: test
    version: "1.0.0"
    url: "not-a-url"
    required: true
"#;
        let reqs: ModRequirements = serde_yaml::from_str(yaml).unwrap();
        assert!(validate_requirements(&reqs).is_err());
    }
}