use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the top-level configuration for an Isolenv isolated environment.
///
/// This structure maps directly to the structure of the `Isolenv` YAML configuration file.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsolenvConfig {
    /// The version of the Isolenv configuration schema.
    pub version: String,
    /// The core environment configuration details.
    pub environment: Environment,
    /// Dependencies required by the environment prior to setup.
    #[serde(default)]
    pub dependencies: Dependencies,
    /// A list of shell commands to execute during the setup phase.
    #[serde(default)]
    pub setup: Vec<String>,
    /// Configuration detailing what command to run once the environment is ready.
    pub run: RunConfig,
}

/// Defines the core characteristics of the isolated environment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    /// A unique or descriptive name for the environment.
    pub name: String,
    /// The base image (e.g., Docker image) to use as the foundation for the environment.
    pub base_image: String,
    /// A mapping of environment variables to be set within the isolated environment.
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
}

/// Specifies packages or libraries that must be installed in the environment.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Dependencies {
    /// System-level packages (e.g., apt packages for Debian/Ubuntu).
    #[serde(default)]
    pub system: Vec<String>,
    /// Python packages to be installed (e.g., via pip).
    #[serde(default)]
    pub python: Vec<String>,
    /// Node.js packages to be installed (e.g., via npm).
    #[serde(default)]
    pub node: Vec<String>,
    /// Rust packages or components to be installed (e.g., via cargo or rustup).
    #[serde(default)]
    pub rust: Vec<String>,
}

/// Configuration for running the primary payload / application in the environment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunConfig {
    /// The command to execute to start the application.
    pub command: String,
}

impl IsolenvConfig {
    /// Parses an `IsolenvConfig` from a specified YAML file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the YAML configuration file.
    ///
    /// # Returns
    ///
    /// * `Ok(IsolenvConfig)` if the file is successfully read and parsed.
    /// * `Err(anyhow::Error)` if the file cannot be opened or if the YAML format is invalid.
    pub fn from_yaml(file_path: &str) -> Result<Self> {
        let f = std::fs::File::open(file_path)
            .with_context(|| format!("Failed to open config file: {}", file_path))?;
        let config: IsolenvConfig = serde_yaml::from_reader(f)
            .with_context(|| "Failed to parse YAML configuration")?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml_parsing() {
        let yaml_str = r#"
version: "1.0"
environment:
  name: "test-env"
  base_image: "alpine:latest"
  env_vars:
    KEY: "VALUE"
run:
  command: "echo test"
"#;
        let config: IsolenvConfig = serde_yaml::from_str(yaml_str).expect("Valid YAML failed to parse");
        assert_eq!(config.version, "1.0");
        assert_eq!(config.environment.name, "test-env");
        assert_eq!(config.environment.base_image, "alpine:latest");
        assert_eq!(config.environment.env_vars.get("KEY").unwrap(), "VALUE");
        assert_eq!(config.run.command, "echo test");
        
        // Assert defaults
        assert!(config.dependencies.system.is_empty());
        assert!(config.setup.is_empty());
    }

    #[test]
    fn test_invalid_field_denied() {
        let yaml_str = r#"
version: "1.0"
bad_field: "should fail"
environment:
  name: "test-env"
  base_image: "alpine"
run:
  command: "echo test"
"#;
        let result: Result<IsolenvConfig, _> = serde_yaml::from_str(yaml_str);
        assert!(result.is_err());
    }
}
