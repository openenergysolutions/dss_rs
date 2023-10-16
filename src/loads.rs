use dss_rs_sys as dss_c;
use crate::dss_result::Result;
use std::ffi::CString;

pub fn set_name(value: &str) -> Result<()> {
    unsafe {
        let c_str = CString::new(value)?;
        dss_c::PVSystems_Set_Name(c_str.into_raw());
    }
    Ok(())
  }

pub fn set_kw(value: f64) {
    unsafe {
        dss_c::Loads_Set_kW(value);
    }
}

pub fn set_kvar(value: f64) {
    unsafe {
        dss_c::Loads_Set_kvar(value);
    }
}

pub fn set_pf(value: f64) {
    unsafe {
        dss_c::Loads_Set_PF(value);
    }
}
