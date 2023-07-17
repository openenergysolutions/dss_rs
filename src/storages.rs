extern crate dss_rs_sys;
use dss_rs_sys as dss_c;


pub fn get_pu_soc() -> f64 {
    unsafe {
       dss_c::Storages_Get_puSOC()
    }
}

pub fn set_pu_soc(value: f64) {
    unsafe {
        dss_c::Storages_Set_puSOC(value);
    }
}

pub fn get_state() -> i32 {
    unsafe {
        dss_c::Storages_Get_State()
    }
}

pub fn set_state(value: i32) {
    unsafe {
        dss_c::Storages_Set_State(value);
    }
}