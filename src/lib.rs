#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::multiple_crate_versions)]

pub mod cli;
pub mod config;
pub mod context_generator;
pub mod file_data;
