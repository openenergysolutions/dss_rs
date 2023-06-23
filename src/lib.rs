#![feature(vec_into_raw_parts)]
/// A non-`unsafe` (re: safe) Rust API for [DSS_Extensions'][DSS_EXTENSIONS] `dss_capi`.
///
/// [DSS_EXTENSIONS]:https://dss-extensions.org/

/// OpenDSS `ActiveClass*` functions.
pub mod active_class;
/// OpenDSS `Bus*` functions.
pub mod bus;
/// OpenDSS `CapControls*` functions.
#[cfg(feature = "unsafe")]
pub mod cap_controls;
/// OpenDSS `Capacitors*` functions.
#[cfg(feature = "unsafe")]
pub mod capacitors;
/// OpenDSS `Ciruit*` functions.
#[cfg(feature = "unsafe")]
pub mod circuit;
/// OpenDSS `CktElement*` functions.
pub mod ckt_element;
/// OpenDSS `DSS*` functions.
pub mod dss;
/// OpenDSS `DSS_Executive*` functions.
#[cfg(feature = "unsafe")]
pub mod dss_executive;
/// Error handling.
pub mod dss_result;
/// OpenDSS `Generators*` functions.
pub mod generators;
/// OpenDSS `Loads*` functions.
pub mod loads;
/// OpenDSS `Reclosers*` functions.
pub mod reclosers;
/// OpenDSS `RegControls*` functions.
pub mod reg_controls;
/// OpenDSS `SwtControls*` functions.
pub mod swt_controls;
/// OpenDSS `Vsource*` functions.
pub mod vsources;
