extern crate dss_rs_sys;
use crate::dss_result::Result;
use dss_rs_sys as dss_c;

pub fn close() {
    unsafe {
        dss_c::Relays_Close();
    }
}

pub fn open() {
    unsafe {
        dss_c::Relays_Open();
    }
}

pub fn set_name(name: &str) -> Result<()> {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe {
        dss_c::Relays_Set_Name(name.as_ptr());
    }
    Ok(())
}
