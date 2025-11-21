use std::path::PathBuf;

use clap::{Parser, command};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Target directory to tokenize
    pub target_dir: PathBuf,

    /// Output file path (overrides config)
    #[arg(short, long)]
    pub out_file: Option<PathBuf>,

    /// Prompt file path (overrides config)
    #[arg(short, long)]
    pub prompt_file: Option<PathBuf>,

    /// Path to config file (default: `$XDG_CONFIG_HOME/tokenize/config.toml`)
    #[arg(short, long = "config")]
    pub config_file: Option<PathBuf>,

    /// Don't respect .gitignore files
    #[arg(long)]
    pub no_gitignore: bool,

    /// Include hidden files
    #[arg(long)]
    pub include_hidden: bool,

    /// Follow symlinks
    #[arg(long)]
    pub follow_symlinks: bool,
}
