mod finder;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <project_root_folder>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    if let Err(err) = finder::read_rust_file(path) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

