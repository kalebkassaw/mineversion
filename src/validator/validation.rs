use super::error::ValidationError;
use super::types::ModRequirements;
use std::collections::HashSet;

/// Validates a URL string
fn validate_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Validates the Minecraft version format
fn validate_minecraft_version(version: &str) -> bool {
    !version.trim().is_empty()
    // Could add more specific validation like regex for version format
    // e.g., version.matches(r"^\d+\.\d+(\.\d+)?$")
}

/// Validates mod requirements and returns all validation errors found
///
/// # Arguments
/// * `reqs` - The mod requirements to validate
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(Vec<ValidationError>)` if validation fails with a list of all errors
///
/// # Examples
/// ```
/// use minecraft_tools::validator::{ModRequirements, validate_requirements};
///
/// let yaml = r#"
/// minecraft_version: "1.20.1"
/// mods:
///   - id: forge
///     version: "47.1.0"
///     required: true
/// "#;
///
/// let reqs: ModRequirements = serde_yaml::from_str(yaml).unwrap();
/// assert!(validate_requirements(&reqs).is_ok());
/// ```
pub fn validate_requirements(reqs: &ModRequirements) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate Minecraft version
    if !validate_minecraft_version(&reqs.minecraft_version) {
        errors.push(ValidationError::InvalidMinecraftVersion);
    }

    // Validate mod list is not empty
    if reqs.mods.is_empty() {
        errors.push(ValidationError::EmptyModList);
    }

    // Track mod IDs to detect duplicates
    let mut seen_ids = HashSet::new();

    // Validate each mod
    for (idx, mod_entry) in reqs.mods.iter().enumerate() {
        // Check for empty ID
        if mod_entry.id.trim().is_empty() {
            errors.push(ValidationError::EmptyId(idx));
        } else {
            // Check for duplicate IDs
            if !seen_ids.insert(mod_entry.id.clone()) {
                errors.push(ValidationError::DuplicateModId(mod_entry.id.clone()));
            }
        }

        // Check for empty version
        if mod_entry.version.trim().is_empty() {
            errors.push(ValidationError::EmptyVersion(idx));
        }

        // Validate URL if present
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
    use crate::validator::types::Mod;

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
        let result = validate_requirements(&reqs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::EmptyId(0))));
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
        let result = validate_requirements(&reqs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidUrl(_, _))));
    }

    #[test]
    fn test_duplicate_mod_ids() {
        let mut reqs = ModRequirements::new("1.20.1".to_string());
        reqs.add_mod(Mod::required("forge".to_string(), "1.0.0".to_string()));
        reqs.add_mod(Mod::required("forge".to_string(), "2.0.0".to_string()));

        let result = validate_requirements(&reqs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::DuplicateModId(_))));
    }

    #[test]
    fn test_empty_mod_list() {
        let reqs = ModRequirements::new("1.20.1".to_string());
        let result = validate_requirements(&reqs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::EmptyModList)));
    }

    #[test]
    fn test_invalid_minecraft_version() {
        let mut reqs = ModRequirements::new("".to_string());
        reqs.add_mod(Mod::required("forge".to_string(), "1.0.0".to_string()));

        let result = validate_requirements(&reqs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidMinecraftVersion)));
    }

    #[test]
    fn test_mod_builder_pattern() {
        let mod_entry = Mod::required("jei".to_string(), "1.0.0".to_string())
            .with_url("https://example.com/jei.jar".to_string());

        assert_eq!(mod_entry.id, "jei");
        assert_eq!(mod_entry.version, "1.0.0");
        assert!(mod_entry.required);
        assert_eq!(mod_entry.url, Some("https://example.com/jei.jar".to_string()));
    }

    #[test]
    fn test_requirements_counts() {
        let mut reqs = ModRequirements::new("1.20.1".to_string());
        reqs.add_mod(Mod::required("forge".to_string(), "1.0.0".to_string()));
        reqs.add_mod(Mod::required("jei".to_string(), "2.0.0".to_string()));
        reqs.add_mod(Mod::optional("optifine".to_string(), "3.0.0".to_string()));

        assert_eq!(reqs.required_count(), 2);
        assert_eq!(reqs.optional_count(), 1);
    }
}