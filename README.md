# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].


## Summary

This crate provides a non-`unsafe` (re: safe) Rust API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

We rely on pre-built binaries, which are included in the `lib` folder. The only target currently supported is `linux_x64`.

This project is unaffiliated with the DSS_Extensions organization in any way.


## Building

```
cargo build
```

[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi

