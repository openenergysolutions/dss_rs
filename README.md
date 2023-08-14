# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].

This crate _must_ be used as a local dependency specified via path. E.g. in `Cargo.toml`

Your project must also have a build script that links the dss_capi lib:
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath=/usr/local/lib");
    println!("cargo:rustc-link-lib=dss_capi");
}
```

```
[dependencies]
dss_rs = { path = "dss_rs" }
```

## Summary

This crate provides a Rusty API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

This project is unaffiliated with the DSS_Extensions organization.


## Building

Builds require the use of a `.devcontainer`

[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi


## Testing

cargo test -- --test-threads=1
