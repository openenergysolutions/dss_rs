fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath=/usr/local/lib");
    println!("cargo:rustc-link-lib=dss_capi");
}
