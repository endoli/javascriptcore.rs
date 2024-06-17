#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
const POTENTIAL_LIBS: [&str; 3] = [
    "javascriptcoregtk-4.1",
    "javascriptcoregtk-4.0",
    "javascriptcoregtk-3.0",
];

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    for l in POTENTIAL_LIBS {
        let r = pkg_config::probe_library(l);
        if r.is_ok() {
            return;
        }
    }
    panic!("libjavascriptcoregtk-4.0, 4.1 or 3.0 must be installed.");
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() {
    panic!("Only macOS and Linux are supported currently.");
}
