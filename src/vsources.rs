extern crate dss_rs_sys;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;
use std::ffi::CString;

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::Vsources_Set_Name(c_str.into_raw());
    }
    Ok(())
}

pub fn set_pu(value: f64) {
    unsafe {
        dss_c::Vsources_Set_pu(value);
    }
}
