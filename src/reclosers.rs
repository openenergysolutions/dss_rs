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