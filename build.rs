fn main() {
    println!("cargo:rustc-link-search=./lib/linux_x64");
    println!("cargo:rustc-link-arg=-L./lib/linux_x64");
    println!("cargo:rustc-link-arg=-I./include");
    println!("cargo:rustc-link-arg=-ldss_capi");
}
