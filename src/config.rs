use std::path::PathBuf;
use std::fs;

use crate::cli::Cli;
use serde::{Deserialize, Serialize};

const DEFAULT_TARGET: &str = ".";
const DEFAULT_OUT: &str = "llm_context.md";
const DEFAULT_PROMPT_FILE: &str = "assets/initial_prompt.md";

#[derive(Debug)]
pub struct Config {
    pub target_dir: PathBuf,
    pub out_file: PathBuf,
    pub prompt_file: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    pub target_dir: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub prompt_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: PathBuf::from(DEFAULT_TARGET),
            out_file: PathBuf::from(DEFAULT_OUT),
            prompt_file: PathBuf::from(DEFAULT_PROMPT_FILE),
        }
    }
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        // Load config file if specified
        let config_file = if let Some(config_path) = &cli.config_file {
            let content = fs::read_to_string(config_path)
                .map_err(|e| Error::ConfigFileReadError(config_path.clone(), e.to_string()))?;
            toml::from_str::<ConfigFile>(&content)
                .map_err(|e| Error::ConfigFileParseError(config_path.clone(), e.to_string()))?
        } else {
            ConfigFile::default()
        };

        // CLI arguments take precedence over config file, which takes precedence over defaults
        let out_file = cli.out_file
            .or(config_file.out_file)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_OUT));
        
        let prompt_file = cli.prompt_file
            .or(config_file.prompt_file)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_PROMPT_FILE));

        Ok(Self {
            target_dir: cli.target_dir,
            out_file,
            prompt_file,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid path to ignore provided: {0}")]
    IgnoreError(String),
    #[error("failed to read config file {0}: {1}")]
    ConfigFileReadError(PathBuf, String),
    #[error("failed to parse config file {0}: {1}")]
    ConfigFileParseError(PathBuf, String),
}
