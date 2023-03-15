use glob::glob;
use std::fs;
use std::io::{self, Read};

pub fn read_files(pattern: &str) -> String {
    let mut content = String::new();

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut file_content = String::new();
                if let Ok(mut file) = fs::File::open(&path) {
                    if let Ok(_) = file.read_to_string(&mut file_content) {
                        content.push_str(&file_content);
                        content.push_str("\n");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error while reading file: {:?}", e);
            }
        }
    }

    if !content.is_empty() {
        content.push_str("Here is the context:\n");
    }

    content
}
