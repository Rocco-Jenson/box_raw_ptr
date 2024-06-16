fn main() {
    cc::Build::new()
        .file("src/allocator.c")
        .compile("allocator");
}
