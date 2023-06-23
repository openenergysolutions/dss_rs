use dss_rs::{capacitors, circuit, ckt_element, dss, reg_controls};

#[test]
fn shared_lib_call() {
    let ret = dss::start(0);
    assert!(ret != 0);
}

#[test]
fn tap_regulator_and_get_reading() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("regcontrol.Reg1").is_ok());
    let cur_tap = reg_controls::get_tap_number();

    // Raise Tap
    let cur_tap = cur_tap + 1;
    reg_controls::set_tap_number(cur_tap);

    assert!(circuit::set_active_element("Transformer.Reg1").is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Lower Tap
    reg_controls::set_tap_number(cur_tap - 3);

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}

#[test]
fn set_capbank_and_get_reading() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("Capacitor.cap1").is_ok());

    // Close CapBank
    assert!(capacitors::set_states(vec![1]).is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Open CapBank
    assert!(capacitors::set_states(vec![0]).is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}
