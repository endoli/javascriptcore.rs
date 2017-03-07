#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(not(any(target_os = "macos")))]
fn main() {
    panic!("Only MacOS is supported currently.");
}
