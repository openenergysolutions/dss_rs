/// A non-`unsafe` (re: safe) Rust API for [DSS_Extensions'][DSS_EXTENSIONS] `dss_capi`.
///
/// [DSS_EXTENSIONS]:https://dss-extensions.org/

// Disables relevant warnings for all bindings.
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/// OpenDSS `linux_x64` target.
#[cfg(feature = "linux_x64")]
pub mod linux_x64;

/// OpenDSS `ActiveClass*` functions.
pub mod active_class;
/// OpenDSS `Bus*` functions.
// TODO: eventually remove
#[allow(unused_variables)]
#[allow(unused_imports)]
pub mod bus;
/// OpenDSS `CapControls*` functions.
pub mod cap_controls;
/// OpenDSS `Capacitors*` functions.
pub mod capacitors;
/// OpenDSS `Ciruit*` functions.
pub mod circuit;
/// OpenDSS `CktElement*` functions.
pub mod ckt_element;
/// OpenDSS `DSS*` functions.
pub mod dss;
/// OpenDSS `DSS_Executive*` functions.
pub mod dss_executive;
/// OpenDSS `Loads*` functions.
pub mod loads;
/// OpenDSS `RegControls*` functions.
pub mod reg_controls;
/// OpenDSS `Transformers*` functions.
pub mod transformers;
