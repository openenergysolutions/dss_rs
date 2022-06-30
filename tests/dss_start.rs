use dss_rs::linux_x64::bindings as dss_c;

// Verify C API bindings can be called.
#[test]
fn test_dss_start() {
    unsafe {
        dss_c::DSS_Start(0);
    }
}
