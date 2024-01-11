// Copyright 2024 Open Energy Solutions Inc.
// 
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(vec_into_raw_parts)]
/// A non-`unsafe` (re: safe) Rust API for [DSS_Extensions'][DSS_EXTENSIONS] `dss_capi`.
///
/// [DSS_EXTENSIONS]:https://dss-extensions.org/
///
///

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
/// OpenDSS `PVSystems*` functions
pub mod pvs;
/// OpenDSS `Reclosers*` functions.
pub mod reclosers;
/// OpenDSS `RegControls*` functions.
pub mod reg_controls;
/// OpenDSS `Relays*` functions.
pub mod relays;
/// OpenDSS `Storages*` functions.
pub mod storages;
/// OpenDSS `SwtControls*` functions.
pub mod swt_controls;
/// OpenDSS `Vsource*` functions.
pub mod vsources;
