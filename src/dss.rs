use dss_rs_sys as dss_c;

pub fn start(code: i32) -> u16 {
    unsafe { dss_c::DSS_Start(code) }
}
