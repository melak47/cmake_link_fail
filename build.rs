fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=cpp_lib");
    let lib_dir = cmake::build("cpp_lib");
    println!("cargo:rustc-link-search=native={}/lib",lib_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=OtherLib")
}