// Compile C file to call C functions in Rust
fn main() {
    cc::Build::new()
        .file("src/example.c")
        .compile("example")
}
