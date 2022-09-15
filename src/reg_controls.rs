extern crate dss_rs_sys;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;
use std::ffi::CString;

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::RegControls_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub fn set_tap_number(value: i32) {
    unsafe {
        dss_c::RegControls_Set_TapNumber(value);
    }
}

pub fn set_max_tap_change(value: i32) {
    unsafe {
        dss_c::RegControls_Set_MaxTapChange(value);
    }
}
