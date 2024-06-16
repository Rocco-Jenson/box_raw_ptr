fn main() {
    cc::Build::new()
        .file("src/allocator.c")
        .shared_flag(true)
        .compile("allocator")
}
