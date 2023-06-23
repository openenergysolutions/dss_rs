extern crate dss_rs_sys;
use dss_rs_sys as dss_c;

pub fn open() {
    unsafe {
        dss_c::Reclosers_Open();
    }
}

pub fn close() {
    unsafe {
        dss_c::Reclosers_Open();
    }
}
