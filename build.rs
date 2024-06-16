fn main() {
    /*
    Build allocator.c into a
    .dll (dynamic link library) file
    */
    cc::Build::new()
        .file("src/allocator.c")
        .shared_flag(true)
        .compile("allocator")
}
