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
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
};

pub fn get_all_names() -> Result<Vec<Vec<String>>> {
    // TODO: use from_raw_parts feature here.
    // Looping was stupid and was done prematurely :)
    unsafe {
        let mut result_ptr: *mut *mut *mut c_char = ptr::null_mut();
        let result_cnt: *mut i32 = ptr::null_mut();
        dss_c::ActiveClass_Get_AllNames(result_ptr, result_cnt);
        if result_ptr == ptr::null_mut() || result_cnt == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let mut all_names: Vec<Vec<String>> = Vec::new();
        while *result_cnt > 0 && result_ptr != ptr::null_mut() {
            let mut names: Vec<String> = Vec::new();
            let raw_arr_ptr: *mut *mut c_char = *result_ptr;
            if raw_arr_ptr != ptr::null_mut() {
                for i in 0.. {
                    let raw_ptr: *const c_char = *(raw_arr_ptr.offset(i)) as *const c_char;
                    if raw_ptr != ptr::null_mut() {
                        let raw_str = CStr::from_ptr(raw_ptr);
                        names.push(String::from(raw_str.to_str()?));
                    }
                }
            }
            all_names.push(names);
            result_ptr = result_ptr.offset(1);
            *result_cnt -= 1;
        }
        Ok(all_names)
    }
}

pub fn get_all_names_gr() {
    unsafe {
        dss_c::ActiveClass_Get_AllNames_GR();
    }
}

pub fn get_first() -> i32 {
    unsafe { dss_c::ActiveClass_Get_First() }
}

pub fn get_next() -> i32 {
    unsafe { dss_c::ActiveClass_Get_Next() }
}

pub fn get_name() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_Name();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::ActiveClass_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub fn get_num_elements() -> i32 {
    unsafe { dss_c::ActiveClass_Get_NumElements() }
}

pub fn get_active_class_name() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_ActiveClassName();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn get_count() -> i32 {
    unsafe { dss_c::ActiveClass_Get_Count() }
}

pub fn get_active_class_parent() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_ActiveClassParent();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}
