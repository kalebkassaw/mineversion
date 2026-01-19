use mod_requirements_validator::{ModRequirements, validate_requirements};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/requirements.yaml>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    
    // Read the file
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    // Parse YAML
    let requirements: ModRequirements = match serde_yaml::from_str(&contents) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            std::process::exit(1);
        }
    };

    // Validate
    match validate_requirements(&requirements) {
        Ok(()) => {
            println!("✓ Validation successful!");
            println!("\nMinecraft version: {}", requirements.minecraft_version);
            println!("Total mods: {}", requirements.mods.len());
            println!("\nMods:");
            for mod_entry in &requirements.mods {
                let required_str = if mod_entry.required { "required" } else { "optional" };
                let url_str = mod_entry.url.as_ref()
                    .map(|u| format!(" ({})", u))
                    .unwrap_or_default();
                println!("  - {} v{} [{}]{}", 
                    mod_entry.id, 
                    mod_entry.version, 
                    required_str,
                    url_str
                );
            }
        }
        Err(errors) => {
            eprintln!("✗ Validation failed with {} error(s):", errors.len());
            for error in errors {
                eprintln!("  - {}", error);
            }
            std::process::exit(1);
        }
    }
}