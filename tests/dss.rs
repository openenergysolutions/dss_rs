use dss_rs::dss;

#[test]
fn dss_start() {
    let ret = dss::start(0);
    assert!(ret != 0);
}
