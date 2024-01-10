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

extern crate dss_rs_sys;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;
use std::ffi::CString;

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::PVSystems_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub fn set_kvar(value: f64) {
    unsafe {
        dss_c::PVSystems_Set_kvar(value);
    }
}

pub fn set_kva_rated(value: f64) {
    unsafe {
        dss_c::PVSystems_Set_kVArated(value);
    }
}

pub fn set_pf(value: f64) {
    unsafe {
        dss_c::PVSystems_Set_PF(value);
    }
}

pub fn set_irradiance(value: f64) {
    unsafe {
        dss_c::PVSystems_Set_Irradiance(value);
    }
}
