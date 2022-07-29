extern crate dss_rs_sys;
use dss_rs::{dss, circuit};

// Verify C API bindings can be called.
#[test]
fn unsafe_dss_start() {
    unsafe {
        let ret = dss_rs_sys::DSS_Start(0);
        assert!(ret != 0);
    }
}

#[test]
fn dss_start() {
    let ret = dss::start(0);
    assert!(ret != 0);
}

#[test]
fn text_set_cmd() {
    if let Err(_) = dss::text_set_command("Redirect ../../cvr_protoype/data/test_feeder/test_feeder.dss") {
        panic!("FAILURE")
    }
}
