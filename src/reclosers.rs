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
use std::ffi::CStr;
use std::os::raw::c_char;

pub fn get_switched_obj() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::Reclosers_Get_SwitchedObj();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn open() {
    unsafe {
        dss_c::Reclosers_Open();
    }
}

pub fn close() {
    unsafe {
        dss_c::Reclosers_Close();
    }
}

pub fn reset() {
    unsafe {
        dss_c::Reclosers_Reset();
    }
}

pub fn get_normal_state() -> i32 {
    unsafe { dss_c::Reclosers_Get_NormalState() }
}

pub fn set_normal_state(value: i32) {
    unsafe {
        dss_c::Reclosers_Set_NormalState(value);
    }
}

pub fn set_name(name: &str) -> Result<()> {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe {
        dss_c::Reclosers_Set_Name(name.as_ptr());
    }
    Ok(())
}
