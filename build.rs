fn main() {
    /*
    Compiles a static library from allocator.c file
    NOTE: when using link attribute in projects, set
    'kind' parameter to correct file state or else linking errors will occur
    example: #[link(name = "example", kind = "static")]
    */
    cc::Build::new()
        .file("src/allocator.c")
        .compile("allocator")
}
