use std::{
    fs,
    path::{Path, PathBuf},
};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

const DEFAULT_TARGET: &str = ".";
const DEFAULT_OUT: &str = "llm_context.md";
const DEFAULT_PROMPT_FILENAME: &str = "prompt.md";
const CONFIG_DIR_NAME: &str = "tokenize";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default = "default_out_file")]
    pub out_file: String,
    #[serde(default = "default_prompt_file")]
    pub prompt_file: String,
}

fn default_out_file() -> String {
    DEFAULT_OUT.to_string()
}

fn default_prompt_file() -> String {
    DEFAULT_PROMPT_FILENAME.to_string()
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            out_file: DEFAULT_OUT.to_string(),
            prompt_file: DEFAULT_PROMPT_FILENAME.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub target_dir: PathBuf,
    pub out_file: PathBuf,
    pub prompt_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: PathBuf::from(DEFAULT_TARGET),
            out_file: PathBuf::from(DEFAULT_OUT),
            prompt_file: get_config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(DEFAULT_PROMPT_FILENAME),
        }
    }
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        // Load config file
        let config_path = if let Some(ref config_path) = cli.config_file {
            config_path.clone()
        } else {
            get_config_file_path()?
        };

        let config_file = load_or_create_config(&config_path)?;

        // CLI args override config file
        let out_file = cli
            .out_file
            .unwrap_or_else(|| PathBuf::from(&config_file.out_file));

        let prompt_file = if let Some(ref pf) = cli.prompt_file {
            pf.clone()
        } else {
            // If config file path is relative, make it relative to config dir
            let config_prompt_path = PathBuf::from(&config_file.prompt_file);
            if config_prompt_path.is_absolute() {
                config_prompt_path
            } else if let Some(config_dir) = get_config_dir() {
                config_dir.join(&config_file.prompt_file)
            } else {
                config_prompt_path
            }
        };

        Ok(Self {
            target_dir: cli.target_dir,
            out_file,
            prompt_file,
        })
    }
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(CONFIG_DIR_NAME))
}

fn get_config_file_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir().ok_or(Error::ConfigDirNotFound)?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
}

fn load_or_create_config(config_path: &Path) -> Result<ConfigFile, Error> {
    if config_path.exists() {
        debug!("Loading config from: {}", config_path.display());
        let contents = fs::read_to_string(config_path)
            .map_err(|e| Error::ConfigReadError(config_path.to_path_buf(), e))?;
        toml::from_str(&contents).map_err(|e| Error::ConfigParseError(e.to_string()))
    } else {
        info!("Config file not found, creating default config at: {}", config_path.display());
        create_default_config(config_path)
    }
}

fn create_default_config(config_path: &Path) -> Result<ConfigFile, Error> {
    let config = ConfigFile::default();

    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::ConfigWriteError(parent.to_path_buf(), e))?;
        
        // Create default prompt file
        let prompt_path = parent.join(&config.prompt_file);
        if !prompt_path.exists() {
            let default_prompt = include_bytes!("../assets/initial_prompt.md");
            fs::write(&prompt_path, default_prompt)
                .map_err(|e| Error::ConfigWriteError(prompt_path.clone(), e))?;
            info!("Created default prompt file at: {}", prompt_path.display());
        }
    }

    // Write config file
    let toml_string = toml::to_string_pretty(&config)
        .map_err(|e| Error::ConfigParseError(e.to_string()))?;
    fs::write(config_path, toml_string)
        .map_err(|e| Error::ConfigWriteError(config_path.to_path_buf(), e))?;

    info!("Created default config at: {}", config_path.display());
    Ok(config)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid path to ignore provided: {0}")]
    IgnoreError(String),
    #[error("could not determine config directory (XDG_CONFIG_HOME or default config dir not available)")]
    ConfigDirNotFound,
    #[error("failed to read config file {0}: {1}")]
    ConfigReadError(PathBuf, std::io::Error),
    #[error("failed to write config file {0}: {1}")]
    ConfigWriteError(PathBuf, std::io::Error),
    #[error("failed to parse config file: {0}")]
    ConfigParseError(String),
}
