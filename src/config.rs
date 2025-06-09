use std::path::PathBuf;

use crate::cli::Cli;

const DEFAULT_TARGET: &str = ".";
const DEFAULT_OUT: &str = "llm_context.md";
const DEFAULT_PROMPT_FILE: &str = "assets/initial_prompt.md";

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
            prompt_file: PathBuf::from(DEFAULT_PROMPT_FILE),
        }
    }
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        let out_file = cli.out_file.unwrap_or_else(|| PathBuf::from(DEFAULT_OUT));
        let prompt_file = cli
            .prompt_file
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
}
