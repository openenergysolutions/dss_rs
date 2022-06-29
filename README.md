# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].


# Summary

This crate provides a non-`unsafe` (re: safe) Rust API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

We rely on pre-built binaries, which are included in the `lib` folder. The only target currently supported is `linux_x64`.

This project is unaffiliated with the DSS_Extensions organization in any way.


# Building

You may or may not have to export the pre-built binaries path to `LD_LIBRARY_PATH`:
```
export LD_LIBRARY_PATH=$PWD/lib/linux_x64:LD_LIBRARY_PATH
```

[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi

