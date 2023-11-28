extern crate dss_rs_sys;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

pub fn start(code: i32) -> u16 {
    unsafe { dss_c::DSS_Start(code) }
}

pub fn text_set_command(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::Text_Set_Command(c_str.into_raw());
    }
    Ok(())
}

pub fn solution_set_mode(mode: i32) {
    unsafe {
        dss_c::Solution_Set_Mode(mode);
    }
}

pub fn solution_solve() {
    unsafe {
        dss_c::Solution_Solve();
    }
}

pub fn solution_init_snap() {
    unsafe {
        dss_c::Solution_InitSnap();
    }
}

pub fn check_error_msg() -> Option<String> {
    unsafe {
        let ctx = dss_c::ctx_Get_Prime();
        if dss_c::ctx_Error_Get_Number(ctx) == 0 {
            return None;
        }
        let raw_ptr: *const c_char = &*dss_c::ctx_Error_Get_Description(ctx);
        let raw_str = CStr::from_ptr(raw_ptr);
        Some(String::from(raw_str.to_str().ok()?))
    }
}

pub fn set_active_class(name: &str) -> Result<i32> {
    unsafe {
        let c_str = CString::new(name)?;
        Ok(dss_c::DSS_SetActiveClass(c_str.into_raw()))
    }
}
