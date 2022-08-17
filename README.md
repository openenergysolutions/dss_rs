# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].

This crate _must_ be used as a local dependency specified via path. E.g. in `Cargo.toml`

```
[dependencies]
dss_rs = { path = "dss_rs" }
```

## Summary

This crate provides a Rusty API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

This project is unaffiliated with the DSS_Extensions organization.


## Building

`cargo build` will only partially build this project for you. You must manually build `dss_capi` (a  dependency of `dss_rs_sys`). Instructions on how to do so are located in `dss_rs_sys`.

#### Sanity Test

Once you have manually built `dss_capi`, make sure you can actually call it.

```
cargo test
```

## Usage

`dss_capi` and `klusolve` (dependencies of `dss_rs_sys`) must either be visible to the linker at runtime or set as an `rpath` at compile time. `build.rs` serves as an example for specifying the `rpath`.


[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi
