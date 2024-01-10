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
use crate::dss_result::DssError;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;
use raw_parts::RawParts;
use std::{convert::TryInto, ffi::CString};
use std::{ptr, slice};

pub unsafe fn get_all_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Capacitors_Get_AllNames(result_ptr, result_count);
}

pub unsafe fn get_all_names_gr() {
    dss_c::Capacitors_Get_AllNames_GR();
}

pub unsafe fn get_first() -> i32 {
    dss_c::Capacitors_Get_First()
}

pub fn get_is_delta() -> u16 {
    unsafe { dss_c::Capacitors_Get_IsDelta() }
}

pub fn get_kv() -> f64 {
    unsafe { dss_c::Capacitors_Get_kV() }
}

pub fn get_kvar() -> f64 {
    unsafe { dss_c::Capacitors_Get_kvar() }
}

pub unsafe fn get_name() -> *mut ::std::os::raw::c_char {
    dss_c::Capacitors_Get_Name()
}

pub unsafe fn get_next() -> i32 {
    dss_c::Capacitors_Get_Next()
}

pub unsafe fn get_num_steps() -> i32 {
    dss_c::Capacitors_Get_NumSteps()
}

pub unsafe fn set_is_delta(value: u16) {
    dss_c::Capacitors_Set_IsDelta(value);
}

pub unsafe fn set_kv(value: f64) {
    dss_c::Capacitors_Set_kV(value);
}

pub unsafe fn set_kvar(value: f64) {
    dss_c::Capacitors_Set_kvar(value);
}

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::Capacitors_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub unsafe fn set_num_steps(value: i32) {
    dss_c::Capacitors_Set_NumSteps(value);
}

pub unsafe fn get_count() -> i32 {
    dss_c::Capacitors_Get_Count()
}

pub unsafe fn add_step() -> u16 {
    dss_c::Capacitors_AddStep()
}

pub unsafe fn subtract_step() -> u16 {
    dss_c::Capacitors_SubtractStep()
}

pub unsafe fn get_available_steps() -> i32 {
    dss_c::Capacitors_Get_AvailableSteps()
}

pub fn get_states() -> Result<Vec<i32>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Capacitors_Get_States(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_states_gr() {
    dss_c::Capacitors_Get_States_GR();
}

pub fn set_states(states: Vec<i32>) -> Result<()> {
    let RawParts { ptr, length, .. } = RawParts::from_vec(states);
    unsafe {
        let ptr = ptr as *mut i32;
        dss_c::Capacitors_Set_States(ptr, length.try_into()?);
    }
    Ok(())
}

pub fn open() {
    unsafe { dss_c::Capacitors_Open() }
}

pub fn close() {
    unsafe {
        dss_c::Capacitors_Close();
    }
}
