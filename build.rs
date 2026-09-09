extern crate napi_build;

fn main() {
  napi_build::setup();

  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  if target_os == "windows" {
    println!("cargo:rerun-if-changed=src/backends/windows/bindings-filter.txt");
    let args = [
        "--out", "src/backends/windows/bindings.rs",
        "--flat",
        "--filter-file", "src/backends/windows/bindings-filter.txt",
    ];
    windows_bindgen::bindgen(args);
  }
}
