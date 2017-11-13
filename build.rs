use std::env;

fn main() {
    let target = env::var("TARGET").expect("TARGET was not set");
    if target.contains("windows-gnu") {
        // make TLS work
        println!("cargo:rustc-link-lib=static=gcc_eh");
        println!("cargo:rustc-link-lib=static=pthread");
    }
}
