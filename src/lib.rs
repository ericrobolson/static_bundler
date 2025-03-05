use base64::prelude::*;
use std::collections::HashMap;

/// Compile static files into a JSON blob.
/// The blob is essentially a map of file paths to their contents.
/// It can be used to serve static files in the browser.
pub fn compile_static_files(src_dir: &str) -> String {
    let mut virtual_files = HashMap::new();

    for entry in walkdir::WalkDir::new(src_dir) {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let path = entry.path();
        let content = std::fs::read(path).unwrap();
        let file_path = path.to_string_lossy().replace(src_dir, "");

        let content = BASE64_STANDARD.encode(content);

        virtual_files.insert(file_path, content);
    }

    serde_json::to_string(&virtual_files).unwrap()
}

/// Decode a JSON blob into a map of file paths to their contents.
pub fn decode_static_files(content: &str) -> HashMap<String, Vec<u8>> {
    let virtual_files: HashMap<String, String> = serde_json::from_str(content).unwrap();

    let mut files = HashMap::new();

    for (path, content) in virtual_files {
        let decoded = BASE64_STANDARD.decode(content).unwrap();
        files.insert(path, decoded);
    }

    files
}
