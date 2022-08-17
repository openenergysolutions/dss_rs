fn main() {
    let pwd = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let mut lib = pwd;
    lib.push_str("/dss_rs_sys/dss_capi/lib/linux_x64");

    println!("cargo:rustc-link-arg=-Wl,-rpath={}", lib);
    println!("cargo:rustc-link-lib=dss_capi");
}