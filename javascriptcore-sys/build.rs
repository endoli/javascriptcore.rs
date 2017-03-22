#[cfg(target_os = "linux")]
extern crate pkg_config;

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
fn main() {
    pkg_config::probe_library("javascriptcoregtk-3.0").unwrap();
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() {
    panic!("Only macOS and Linux are supported currently.");
}
