// Parts of this are based on the work on alexcrichton, here:
// https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs
//
// This has been tested on Ubuntu and it assumes that CMake is available.
// Patches are welcome to help make it work on other operating systems!

extern crate "pkg-config" as pkg_config;

use std::io;
use std::io::Command;
use std::io::fs::{mkdir,PathExtensions};
use std::io::process::InheritFd;
use std::os;

fn main() {
    // Do nothing if this package is already provided by the system.
    if pkg_config::find_library("uchardet").is_ok() { return; }

    // Get our configuration from our environment.
    let mut cxxflags = os::getenv("CXXFLAGS").unwrap_or(String::new());
    let target = os::getenv("TARGET").unwrap();
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    // Fix up our build flags.
    if target.contains("i686") {
        cxxflags.push_str(" -m32");
    } else if target.as_slice().contains("x86_64") {
        cxxflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cxxflags.push_str(" -fPIC");
    }

    // Make a build directory.
    let build = dst.join("build");
    if !build.is_dir() {
        mkdir(&build, io::USER_DIR).unwrap();
    }

    // Set up our CMake command.
    run(Command::new("cmake")
            .cwd(&dst.join("build"))
            .arg(src.join("uchardet"))
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg(format!("-DCMAKE_INSTALL_PREFIX={}", dst.display()))
            .arg(format!("-DCMAKE_CXX_FLAGS={}", cxxflags)));
    
    // Run our make command.
    run(Command::new("make")
            .cwd(&dst.join("build"))
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
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());
}
