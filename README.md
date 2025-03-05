# static_bundler

A tool to bundle files into a JSON blob with each key representing a file path and the value being the file content.

An example use case is a HTTP server that needs to serve HTML pages. 


## Example Usage

Add to cargo:
```
[build-dependencies]
static_bundler = "0.1.0"
```

Add to `build.rs`:
```
fn main() {
    println!("Building webview static files");
    const SRC_DIR: &str = "static/";
    const DEST_FILE: &str = ".static_files.json";

    let json_string = static_bundler::compile_static_files(SRC_DIR);

    std::fs::write(DEST_FILE, json_string).unwrap();
}
```

Add to your code:
```
let json_blob = include_str!("../.static_files.json");
let files = static_bundler::decode_static_files(json_blob);
```