use std::path::PathBuf;

use clap::{Parser, command};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    pub target_dir: PathBuf,

    #[arg(short, long)]
    pub out_file: Option<PathBuf>,

    #[arg(short, long)]
    pub prompt_file: Option<PathBuf>,

    #[arg(long)]
    pub no_gitignore: bool,

    #[arg(long)]
    pub include_hidden: bool,

    #[arg(long)]
    pub follow_symlinks: bool,

    #[arg(long)]
    pub include_binary: bool,
}
