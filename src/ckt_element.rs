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
use crate::dss_result::{DssError, Result};
use dss_rs_sys as dss_c;
use std::{ffi::CStr, os::raw::c_char, ptr, slice};

pub fn enable() {
    unsafe {
        dss_c::CktElement_Set_Enabled(true.into());
    }
}

pub fn disable() {
    unsafe {
        dss_c::CktElement_Set_Enabled(false.into());
    }
}

pub fn get_enabled() -> u16 {
    unsafe { dss_c::CktElement_Get_Enabled() }
}

pub fn open(term: i32, phs: i32) {
    unsafe {
        dss_c::CktElement_Open(term, phs);
    }
}

pub fn close(term: i32, phs: i32) {
    unsafe {
        dss_c::CktElement_Close(term, phs);
    }
}

pub fn get_powers() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_Powers(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_voltages_mag_ang() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_VoltagesMagAng(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_currents_mag_ang() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_CurrentsMagAng(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_total_powers() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_TotalPowers(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn is_open(term: i32, phs: i32) -> u16 {
    unsafe { dss_c::CktElement_IsOpen(term, phs) }
}

#[allow(unused_mut)]
pub fn get_all_property_names() -> Vec<String> {
    unsafe {
        let ctx = dss_c::ctx_Get_Prime();
        let mut result_ptr: *mut *mut c_char = ptr::null_mut();
        let mut v: [i32; 4] = [0, 0, 0, 0];
        dss_c::ctx_CktElement_Get_AllPropertyNames(ctx, &mut result_ptr, v.as_ptr() as *mut i32);
        let vec_strs = slice::from_raw_parts(result_ptr, v[0] as usize)
            .to_vec()
            .iter()
            .filter_map(|ptr| CStr::from_ptr(*ptr).to_str().ok().map(|s| s.to_string()))
            .collect();
        vec_strs
    }
}
