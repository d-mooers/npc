use glob::glob;
use std::fs;
use std::io::Read;

pub fn read_files(pattern: &str) -> String {
    let mut content = String::new();

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut file_content = String::new();
                if let Ok(mut file) = fs::File::open(&path) {
                    if let Ok(_) = file.read_to_string(&mut file_content) {
                        content.push_str(format!("BEGIN FILE: {}\n", path.display()).as_str());
                        content.push_str(format!("```\n{}\n```\n", file_content).as_str());
                        content.push_str(format!("END FILE {}\n\n", path.display()).as_str());
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
