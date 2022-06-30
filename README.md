# `dss_rs`

Early-stage crate providing access to the OpenDSS C API from [DSS_Extensions][DSS_EXTENSIONS].


## Summary

This crate provides a non-`unsafe` (re: safe) Rust API on top of `unsafe` bindings to the OpenDSS C API by DSS_Extensions.

We rely on pre-built binaries, which are included in the `lib` folder. The only target currently supported is `linux_x64`.

This project is unaffiliated with the DSS_Extensions organization.


## Building

As easy as `cargo build`

## Sanity Test

You will want to perform a sanity test to make sure that the `dss_capi` libs can actually be called.
This requires setting the `$LD_LIBRARY_PATH` environment variable:
```
LD_LIBRARY_PATH=$PWD/lib/linux_x64:$LD_LIBRARY_PATH cargo test
```

[DSS_EXTENSIONS]:https://github.com/dss-extensions/dss_capi


