// Parts of this are based on the work on alexcrichton, here:
// https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs
//
// This has been tested on Ubuntu and it assumes that CMake is available.
// Patches are welcome to help make it work on other operating systems!

extern crate pkg_config;
extern crate cmake;

fn main() {
    // Do nothing if this package is already provided by the system.
    if pkg_config::find_library("uchardet").is_ok() { return; }

    // Build uchardet ourselves
    let dst = cmake::build("uchardet");

    // Decide how to link our C++ runtime.  Feel free to submit patches
    // to make this work on your platform.  Other likely options are "c++"
    // and "c++abi" depending on OS and compiler.
    let cxx_abi = "stdc++";

    // Print out link instructions for Cargo.
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=uchardet");
    println!("cargo:rustc-flags=-l {}", cxx_abi);
}
