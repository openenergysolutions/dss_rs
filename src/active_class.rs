#[cfg(feature = "linux_x64")]
use crate::linux_x64::bindings as dss_c;
use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::str::Utf8Error;

pub fn get_all_names() -> Vec<String> {
    todo!();
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

pub fn get_name() -> Result<String, Utf8Error> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_Name();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn set_name(value: &str) -> Result<(), NulError> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::ActiveClass_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub fn get_num_elements() -> i32 {
    unsafe { dss_c::ActiveClass_Get_NumElements() }
}

pub fn get_active_class_name() -> Result<String, Utf8Error> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_ActiveClassName();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn get_count() -> i32 {
    unsafe { dss_c::ActiveClass_Get_Count() }
}

pub fn get_active_class_parent() -> Result<String, Utf8Error> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::ActiveClass_Get_ActiveClassParent();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}
