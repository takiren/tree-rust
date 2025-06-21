use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tree-rust")]
#[command(about = "A Rust implementation of tree command")]
#[command(version = "0.1.0")]
pub struct Args {
    /// Directory path to display (default: current directory)
    pub path: Option<PathBuf>,

    /// Maximum depth to display
    #[arg(short = 'd', long = "depth")]
    pub depth: Option<usize>,

    /// Show files only
    #[arg(short = 'f', long = "files-only")]
    pub files_only: bool,

    /// Show directories only
    #[arg(short = 'D', long = "dirs-only")]
    pub dirs_only: bool,
}
