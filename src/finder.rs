use std::fs;
use std::path::Path;

pub fn read_rust_file(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);

    if !path.is_file() {
        return Err(format!("Provided path is not a file: {}", file_path));
    }

    if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
        return Err(format!("Provided file is not a Rust file: {}", file_path));
    }

    println!("Found Rust file at: {}", path.display());

    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // println!("--- File contents ---\n{}", contents);

    Ok(contents)
}

