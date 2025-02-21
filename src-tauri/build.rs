// use std::env;
// use std::path::Path;
fn main() {
    // let dir = env::var("OPENCV_LINK_PATHS").unwrap();
    // println!("cargo:rustc-link-search=native={}/lib", dir);
    tauri_build::build()
}
