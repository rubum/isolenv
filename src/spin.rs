use crate::config::IsolenvConfig;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Generates the initialization script for the isolated environment.
pub fn generate_script(config: &IsolenvConfig) -> String {
    let mut script = String::new();

    // Install OS-level packages required by the environment
    if !config.dependencies.system.is_empty() {
        script.push_str("apt-get update -y && apt-get install -y ");
        script.push_str(&config.dependencies.system.join(" "));
        script.push_str(" && ");
    }

    // Install standard Python packages
    if !config.dependencies.python.is_empty() {
        script.push_str("pip3 install ");
        script.push_str(&config.dependencies.python.join(" "));
        script.push_str(" && ");
    }

    // Install global Node.js packages
    if !config.dependencies.node.is_empty() {
        script.push_str("npm install -g ");
        script.push_str(&config.dependencies.node.join(" "));
        script.push_str(" && ");
    }

    // Install Rust crates globally
    if !config.dependencies.rust.is_empty() {
        script.push_str("cargo install ");
        script.push_str(&config.dependencies.rust.join(" "));
        script.push_str(" && ");
    }

    // Execute custom initialization scripts
    for cmd in &config.setup {
        script.push_str(cmd);
        script.push_str(" && ");
    }

    // Append the main application payload to the execution chain
    script.push_str(&config.run.command);

    script
}

/// Orchestrates the spinning up of the isolated environment using Docker.
pub fn up(config: &IsolenvConfig) -> Result<()> {
    let script = generate_script(config);

    println!("Generated init script:");
    println!("  {}", script);

    // Configure the Docker runtime container
    let mut docker_cmd = Command::new("docker");

    // Assign the static parameters for the Docker container
    docker_cmd.args(["run", "--rm", "-i", "--name", &config.environment.name]);

    // Add dynamic environment variables parsed from configuration
    for (key, value) in &config.environment.env_vars {
        docker_cmd.args(["-e", &format!("{}={}", key, value)]);
    }

    // Specify the base image and the run script execution command
    docker_cmd
        .arg(&config.environment.base_image)
        .args(["/bin/sh", "-c", &script]);

    println!("\nSpawning Docker container...");

    // Spawn the environment and stream standard I/O
    let mut child = docker_cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| "Failed to spawn Docker process. Is Docker daemon running?")?;

    let status = child
        .wait()
        .with_context(|| "Failed to wait on Docker process")?;

    if !status.success() {
        anyhow::bail!("Environment execution failed with {}", status);
    } else {
        println!("Environment execution finished successfully.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Dependencies, Environment, RunConfig};
    use std::collections::HashMap;

    #[test]
    fn test_generate_script() {
        let config = IsolenvConfig {
            version: "1.0".to_string(),
            environment: Environment {
                name: "test".to_string(),
                base_image: "ubuntu".to_string(),
                env_vars: HashMap::new(),
            },
            dependencies: Dependencies {
                system: vec!["curl".to_string()],
                python: vec!["requests".to_string()],
                node: vec![],
                rust: vec![],
            },
            setup: vec!["echo hello".to_string()],
            run: RunConfig {
                command: "python3 main.py".to_string(),
            },
        };

        let script = generate_script(&config);
        let expected = "apt-get update -y && apt-get install -y curl && pip3 install requests && echo hello && python3 main.py";
        assert_eq!(script, expected);
    }
}
