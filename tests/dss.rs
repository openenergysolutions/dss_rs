use dss_rs::{capacitors, circuit, ckt_element, dss, loads, pvs, reg_controls};

#[test]
fn shared_lib_call() {
    let ret = dss::start(0);
    assert!(ret != 0);
}

#[test]
fn tap_regulator() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("regcontrol.Reg1").is_ok());

    let cur_tap = reg_controls::get_tap_number();

    // Raise Tap
    let cur_tap = cur_tap + 1;
    reg_controls::set_tap_number(cur_tap);
    dss::solution_solve();

    assert!(circuit::set_active_element("Transformer.Reg1").is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Lower Tap
    reg_controls::set_tap_number(cur_tap - 3);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    let vll = dss_rs::bus::get_vll();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
    assert!(vll.is_ok());
}

#[test]
fn closeopen_capbank() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("Capacitor.cap1").is_ok());

    // Close CapBank
    capacitors::close();
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Open CapBank
    capacitors::open();
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    let vll = dss_rs::bus::get_vll();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
    assert!(vll.is_ok());
}

#[test]
fn closeopen_switch() {
    // Breakers are treated like switches in OpenDSS;
    // thus opening/closing a breaker is the same as below.
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("Line.671692").is_ok());

    // Close Switch
    ckt_element::close(1, 1);
    ckt_element::close(1, 2);
    ckt_element::close(1, 3);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    // check if phs_c is open
    let phs_a = ckt_element::is_open(1, 1);
    let phs_b = ckt_element::is_open(1, 2);
    let phs_c = ckt_element::is_open(1, 3);
    println!("\nphs_a: {}, phs_b: {}, phs_c: {}", phs_a, phs_b, phs_c);
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Open Switch
    ckt_element::open(1, 1);
    ckt_element::open(1, 2);
    ckt_element::open(1, 3);
    dss::solution_solve();
    let phs_a = ckt_element::is_open(1, 1);
    let phs_b = ckt_element::is_open(1, 2);
    let phs_c = ckt_element::is_open(1, 3);
    println!("phs_a: {}, phs_b: {}, phs_c: {}", phs_a, phs_b, phs_c);
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}

#[ignore]
#[test]
fn closeopen_recloser() {
    todo!("tests/data/IEEE13Nodeckt.dss does not contain 'Recloser' device")
}

#[ignore]
#[test]
fn set_generator() {
    todo!("tests/data/IEEE13Nodeckt.dss does not contain 'Generator' device")
}

#[test]
fn set_load() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("Load.671").is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Set P, Q, PF
    loads::set_kw(5.0);
    loads::set_kvar(5.9);
    loads::set_pf(3.0);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}

#[test]
fn set_storage() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("Storage.s680").is_ok());

    dss_rs::storages::set_kw(200.0).expect("Failed to set kW for Storage.s680");
    dss_rs::storages::set_kvar(10.0).expect("Failed to set kVar for Storage.s680");
    dss_rs::storages::set_pf(50.0).expect("Failed to set PF for Storage.s680");

    dss::solution_solve();
    let kw = dss_rs::storages::get_kw();
    let kvar = dss_rs::storages::get_kvar();
    let pf = dss_rs::storages::get_pf();
    assert!(kw.is_ok());
    assert!(kvar.is_ok());
    assert!(pf.is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();

    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Charge storage
    dss_rs::storages::set_state(-1);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Discharge storage
    dss_rs::storages::set_state(1);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}

#[test]
fn set_pv() {
    assert!(dss::start(0) != 0);
    assert!(dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss").is_ok());
    assert!(circuit::set_active_element("PVSystem.PV1").is_ok());

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());

    // Set P, Q, PF
    pvs::set_kvar(5.9);
    pvs::set_kva_rated(5.0);
    pvs::set_pf(3.0);
    dss::solution_solve();

    let voltages = ckt_element::get_voltages_mag_ang();
    let currents = ckt_element::get_powers();
    assert!(voltages.is_ok());
    assert!(currents.is_ok());
}
