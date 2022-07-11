# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].

This crate _must_ be used as a local dependency specified via path. E.g. in `Cargo.toml`

```
[dependencies]
dss_rs = { path = "dss_rs" }
```

## Summary

This crate provides a non-`unsafe` (re: safe) Rust API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

This project is unaffiliated with the DSS_Extensions organization.


## Building

Using this project will require you to build `dss_rs_sys`. Instructions on how can be found in the project folder.

## Running

Using this crate requires setting `LD_LIBRARY_PATH`. E.g.
```
LD_LIBRARY_PATH=dss_rs_sys/dss_capi/lib/linux_x64 cargo test
```

[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi


