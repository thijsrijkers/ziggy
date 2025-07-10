use std::fs;
use std::path::Path;

pub fn find_main_rs(root: &str) -> Result<(), String> {
    let root_path = Path::new(root);

    if !root_path.is_dir() {
        return Err(format!("Provided path is not a directory: {}", root));
    }

    let src_path = root_path.join("src");
    if !src_path.is_dir() {
        return Err(format!("No 'src' directory found in: {}", root));
    }

    let main_rs = src_path.join("main.rs");
    if !main_rs.is_file() {
        return Err(format!("No main.rs file found in: {}", src_path.display()));
    }

    println!("Found main.rs at: {}", main_rs.display());

    let contents = fs::read_to_string(&main_rs)
        .map_err(|e| format!("Failed to read main.rs: {}", e))?;

    println!("--- main.rs contents ---\n{}", contents);

    Ok(())
}

