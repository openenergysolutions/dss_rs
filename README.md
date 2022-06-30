# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].


## Summary

This crate provides a non-`unsafe` (re: safe) Rust API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

We rely on pre-built binaries, which are included in the `lib` folder. The only target currently supported is `linux_x64`.

This project is unaffiliated with the DSS_Extensions organization.


## Building

As easy as `cargo build`

## Sanity Test

If everything has built successfully, you will want to perform a sanity test to
make sure that the dynamically linked `dss_capi` libs can actually be called.

First, export the pre-built binaries path to `LD_LIBRARY_PATH`:
```
export LD_LIBRARY_PATH=$PWD/lib/linux_x64:LD_LIBRARY_PATH
```

Then run `cargo test dss_start`.



[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi


