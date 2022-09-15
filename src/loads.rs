use dss_rs_sys as dss_c;

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
