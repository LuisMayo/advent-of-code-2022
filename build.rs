use std::fs::{self, FileType};

fn main() {
    if !cfg!(debug_assertions) {
        let folders = fs::read_dir("./src/").unwrap();
        for folder_result in folders {
            let folder = folder_result.unwrap();
            if folder.file_type().unwrap().is_dir() {
                // folder.
            }
        }
    }
}