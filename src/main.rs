mod config;
mod spin;

use crate::config::IsolenvConfig;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "isolenv")]
#[command(about = "A Rust-based engine to spin up isolated environments")]
struct Cli {
    /// Path to the Isolenv configuration file
    #[arg(short, long, global = true, default_value = "isolenv.yaml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lifecycle management for isolated environments
    Spin {
        #[command(subcommand)]
        action: SpinCommands,
    },
}

#[derive(Subcommand, Debug)]
enum SpinCommands {
    /// Spins up the environment
    Up,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Spin { action } => match action {
            SpinCommands::Up => {
                println!("Loading configuration from: {}", cli.config);

                let config_data = IsolenvConfig::from_yaml(&cli.config)
                    .with_context(|| "Failed to load environment configuration")?;

                println!("Successfully parsed configuration.");
                println!("\nInitializing environment: {}", config_data.environment.name);
                println!("Base image: {}", config_data.environment.base_image);

                spin::up(&config_data).with_context(|| "Failed to spin up environment")?;
            }
        },
    }

    Ok(())
}
