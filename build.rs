fn main() {
    // We assume that dc1394-2 library is on the default linker search path.
    println!("cargo:rustc-link-lib=dc1394");
}
