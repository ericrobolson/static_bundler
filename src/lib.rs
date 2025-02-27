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
        let content = std::fs::read_to_string(path).unwrap();
        let file_path = path.to_string_lossy().replace(src_dir, "");
        virtual_files.insert(file_path, content);
    }

    serde_json::to_string(&virtual_files).unwrap()
}
