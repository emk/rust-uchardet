extern crate "pkg-config" as pkg_config;

fn main() {
    match pkg_config::find_library("uchardet") {
        Ok(()) => return,
        Err(..) => panic!("Building uchardet from source not yet supported")
    }
}
