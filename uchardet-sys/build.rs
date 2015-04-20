// Parts of this are based on the work on alexcrichton, here:
// https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs
//
// This has been tested on Ubuntu and it assumes that CMake is available.
// Patches are welcome to help make it work on other operating systems!

extern crate pkg_config;

use std::env;
use std::fs::{PathExt, create_dir};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    // Do nothing if this package is already provided by the system.
    if pkg_config::find_library("uchardet").is_ok() { return; }

    // Get our configuration from our environment.
    let mut cxxflags = env::var("CXXFLAGS").unwrap_or(String::new());
    let target = env::var("TARGET").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    // Fix up our build flags.
    if target.contains("i686") {
        cxxflags.push_str(" -m32");
    } else if target.contains("x86_64") {
        cxxflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cxxflags.push_str(" -fPIC");
    }

    // Make a build directory.
    let build = dst.join("build");
    // This isn't ideal but until is_dir() is stable just always try to create
    // the build directory. If it exists and error is returned, which is
    // ignored.
    create_dir(&build);

    // Set up our CMake command.
    run(Command::new("cmake")
            .current_dir(&dst.join("build"))
            .arg(&src.join("uchardet"))
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg(&format!("-DCMAKE_INSTALL_PREFIX={}", dst.display()))
            .arg(&format!("-DCMAKE_CXX_FLAGS={}", cxxflags)));
    
    // Run our make command.
    run(Command::new("make")
            .current_dir(&dst.join("build"))
            .arg("install"));

    // Decide how to link our C++ runtime.  Feel free to submit patches
    // to make this work on your platform.  Other likely options are "c++"
    // and "c++abi" depending on OS and compiler.
    let cxx_abi = "stdc++";

    // Print out link instructions for Cargo.
    println!("cargo:rustc-flags=-L {} -l uchardet:static -l {}",
             dst.join("lib").display(), cxx_abi);
    println!("cargo:root={}", dst.display());
}

// Run an external build command.
fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
