// This assumes that either uchardet dev packages are installed where
// `pkg_config` can find them, or that CMake and supporting tools are
// available.  Patches are welcome to help make it work on other operating
// systems!

extern crate pkg_config;
extern crate cmake;

use cmake::Config;

fn main() {
    // Do nothing if this package is already provided by the system.
    if pkg_config::find_library("uchardet").is_ok() { return; }

    // Build uchardet ourselves
    let mut config = Config::new("uchardet");

    // Mustn't build the binaries as they aren't compatible with Windows
    // and cause a compiler error
    config.define("BUILD_BINARY", "OFF");
    config.define("BUILD_STATIC", "ON");
    config.define("BUILD_SHARED_LIBS", "OFF");

    if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
        // FIXME: This is only needed on newer versions of gcc (>5 ?); Older
        //        versions fail with "unrecognized command line option" and
        //        abort the build; We need to somehow detect the compiler version
        // Disable sized deallocation as we're unable to link when it's enabled
        config.cxxflag("-fno-sized-deallocation");
    }

    let dst = config.build();

    // Print out link instructions for Cargo.
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=uchardet");

    if !(cfg!(target_os = "windows") && cfg!(target_env = "msvc")) {
        // Not needed on windows-msvc

        // Decide how to link our C++ runtime.  Feel free to submit patches
        // to make this work on your platform.  Other likely options are "c++"
        // and "c++abi" depending on OS and compiler.
        let cxx_abi = "stdc++";
        println!("cargo:rustc-flags=-l {}", cxx_abi);
    }
}
