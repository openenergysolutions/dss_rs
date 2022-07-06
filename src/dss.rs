#[cfg(feature = "linux_x64")]
use crate::linux_x64::bindings as dss_c;

pub fn start(code: i32) -> u16 {
    unsafe { dss_c::DSS_Start(code) }
}
