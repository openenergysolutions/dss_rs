fn main() {
    let pwd = std::env::var("PWD").expect("No working directory?");
    
    let mut lib = pwd.clone(); 
    lib.push_str("/lib/linux_x64");
    
    let mut link_search = String::from("cargo:rustc-link-search=-L");
    link_search.push_str(&lib);
    println!("{}", &link_search);

    let mut link_arg = String::from("cargo:rustc-link-arg=-L");
    link_arg.push_str(&lib);
    println!("{}", &link_arg);
    
    let mut include = String::from("cargo:rustc-link-arg=-I");
    include.push_str(&pwd);
    include.push_str("/include");
    println!("{}", &include);

    println!("cargo:rustc-link-arg=-ldss_capi");
}
