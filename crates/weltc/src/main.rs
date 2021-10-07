use std::env;
use std::path::PathBuf;

use clap::{Clap, IntoApp};

mod compile;

fn main() {
    if env::args_os().count() <= 1 {
        Opts::into_app().print_help().unwrap();
    }

    let opts = Opts::parse();

    compile::compile(&opts.file, opts.out_dir.as_deref());
}

/// The Welt compiler
#[derive(Clap)]
struct Opts {
    file: PathBuf,
    #[clap(long)]
    out_dir: Option<PathBuf>,
}
