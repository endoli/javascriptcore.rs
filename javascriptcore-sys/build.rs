#[cfg(target_os = "linux")]
extern crate pkg_config;

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if let Ok(_) = std::env::var("DOCS_RS") {
        return;
    }
    let r = pkg_config::probe_library("javascriptcoregtk-3.0");
    if r.is_err() {
        let r = pkg_config::probe_library("javascriptcoregtk-4.0");
        if r.is_err() {
            panic!("libjavascriptcoregtk-3.0-dev or -4.0-dev must be installed.")
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() {
    panic!("Only macOS and Linux are supported currently.");
}
