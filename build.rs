extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .include("src/c_code")
        .file("src/c_code/helper.cc")
        .flag_if_supported("-std=c++17")
        .shared_flag(true)
        .compile("libdir_space.so");

    println!("cargo:rustc-link-search=all=src");
    //println!("cargo:rustc-link-lib=dylib=doubler.a");
}