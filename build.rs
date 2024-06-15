fn main() {
    cc::Build::new()
        .file("src/allocator.c")
        .compile("allocator");
    
    println!("cargo:rerun-if-changed=src/my_c_code.c");
}
