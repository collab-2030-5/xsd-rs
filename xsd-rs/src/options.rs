use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xsd-rs")]
#[command(version)]
#[command(about = "Reads model of xml from json and emits rust bindings", long_about = None)]
pub(crate) struct Options {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Generate Rust from an xsd + configuration
    Generate(GenerateOptions),
    /// Do a light parsing of the XSD and output its parsing
    Inspect(InspectOptions),
}

impl Options {
    pub(crate) fn log_level(&self) -> tracing::Level {
        match &self.command {
            Commands::Generate(x) => {
                if x.debug {
                    tracing::Level::DEBUG
                } else {
                    tracing::Level::INFO
                }
            }
            Commands::Inspect(_) => tracing::Level::INFO,
        }
    }
}

#[derive(Debug, Parser)]
pub(crate) struct InspectOptions {
    /// Input XSD file
    #[arg(short = 'i', long = "input")]
    pub(crate) input: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct GenerateOptions {
    /// Input XSD file
    #[arg(short = 'i', long = "input")]
    pub(crate) inputs: Vec<PathBuf>,
    /// Config file
    #[arg(short = 'c', long = "config")]
    pub(crate) config: PathBuf,
    /// Rust output directory
    #[arg(short = 'o', long = "output")]
    pub(crate) output: PathBuf,
    /// Remove the output directory if it exists
    #[arg(short = 'r', long = "remove", default_value_t = false)]
    pub(crate) remove_dir: bool,
    /// Enable debug level logging
    #[arg(short = 'd', long = "debug", default_value_t = false)]
    pub(crate) debug: bool,
}

impl Options {
    pub(crate) fn get() -> Self {
        Self::parse()
    }
}
