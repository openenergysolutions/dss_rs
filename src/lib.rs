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
// Attributes indicate module in-progress
#[allow(unused_variables)]
#[allow(unused_imports)]
pub mod bus;
/// OpenDSS `CapControls*` functions.
#[cfg(feature = "cap_controls")]
pub mod cap_controls;
/// OpenDSS `Capacitors*` functions.
#[cfg(feature = "capacitors")]
pub mod capacitors;
/// OpenDSS `Ciruit*` functions.
#[cfg(feature = "circuit")]
pub mod circuit;
/// OpenDSS `CktElement*` functions.
#[cfg(feature = "ckt_element")]
pub mod ckt_element;
/// OpenDSS `DSS*` functions.
#[cfg(feature = "dss")]
pub mod dss;
/// OpenDSS `DSS_Executive*` functions.
#[cfg(feature = "dss_executive")]
pub mod dss_executive;
/// OpenDSS error handling.
pub mod dss_result;
/// OpenDSS `Loads*` functions.
#[cfg(feature = "loads")]
pub mod loads;
/// OpenDSS `RegControls*` functions.
#[cfg(feature = "reg_controls")]
pub mod reg_controls;
/// OpenDSS `Transformers*` functions.
#[cfg(feature = "transformers")]
pub mod transformers;
