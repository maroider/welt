use std::fs;
use std::path::Path;

mod parse;

pub fn compile(file: &Path, _out_dir: Option<&Path>) {
    let file = fs::OpenOptions::new().read(true).open(file).unwrap();
    parse::parse(file);
}
