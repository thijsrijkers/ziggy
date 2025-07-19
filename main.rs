mod finder;
mod token;
mod tokenizer;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <project_root_folder>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let content_result = finder::read_rust_file(path);

    match content_result {
        Ok(content) => {
            let tokens = tokenizer::tokenize(&content);
            println!("{:#?}", tokens); 
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}

