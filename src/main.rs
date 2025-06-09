#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
use std::path::PathBuf;

use clap::Parser;
use env_logger::Env;
use ignore::WalkBuilder;
use log::debug;
use tokenize_cli::{cli::Cli, config::Config, context_generator::ContextGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = init();

    let walk = WalkBuilder::new(PathBuf::from("."))
        .follow_links(cli.follow_symlinks)
        .hidden(!cli.include_hidden)
        .git_ignore(!cli.no_gitignore)
        .build_parallel();

    let config = Config::from_cli(cli)?;

    ContextGenerator::new(&config)?.generate(walk)?;

    Ok(())
}

fn init() -> Cli {
    let env = Env::default().filter_or("RUST_LOG", "error");
    env_logger::init_from_env(env);

    let cli = Cli::parse();
    debug!("parsed cli args: {cli:#?}");

    cli
}
