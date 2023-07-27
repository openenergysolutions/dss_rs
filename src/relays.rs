extern crate dss_rs_sys;
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