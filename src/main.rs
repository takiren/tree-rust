use clap::Parser;
use std::env;
use tree_rust::{Args, TreeDisplay};

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| env::current_dir().unwrap());

    let tree_display = TreeDisplay::new(args.files_only, args.dirs_only, args.depth);

    if let Err(e) = tree_display.display(&path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
