use std::path::PathBuf;

use clap::{Parser, ValueEnum, command};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum EncodingModel {
    /// GPT-5, GPT-4.1, GPT-4o, o4, o3, and o1 models
    O200kBase,
    /// `ChatGPT` models, text-embedding-ada-002
    Cl100kBase,
    /// Code models, text-davinci-002, text-davinci-003
    P50kBase,
    /// Edit models like text-davinci-edit-001
    P50kEdit,
    /// GPT-3 models like davinci
    R50kBase,
}

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

    /// `OpenAI` encoding model to use for tokenization (only used with --show-tokens)
    #[arg(short = 'm', long, value_enum, default_value = "o200k-base")]
    pub model: EncodingModel,

    /// Show token counts for each file and total (uses `OpenAI` tokenization)
    #[arg(short = 't', long)]
    pub show_tokens: bool,
}
