fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath=dss_rs_sys/dss_capi/lib/linux_x64");
    println!("cargo:rustc-link-lib=dss_capi");
}
