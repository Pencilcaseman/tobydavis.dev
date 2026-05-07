use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, env = "CONTENT_ROOT")]
    pub content_root: Option<PathBuf>,
}

static ARGS: std::sync::OnceLock<Args> = std::sync::OnceLock::new();

pub fn init(args: Args) {
    ARGS.set(args).expect("Args already initialized");
}

pub fn get() -> &'static Args {
    ARGS.get().expect("Args not initialized — call cli::init() in main")
}
