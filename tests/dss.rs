use dss_rs::{circuit, ckt_element, dss, reg_controls};

#[test]
fn shared_lib_call() {
    let ret = dss::start(0);
    assert!(ret != 0);
}

// #[test]
// fn open_breaker_and_get_reading() {
//     todo!();
// }

// #[test]
// fn close_breaker_and_get_reading() {
//     todo!()
// }

// #[test]
// fn open_capbank_and_get_reading() {
//     todo!()
// }

// #[test]
// fn enable_load_and_get_reading() {
//     todo!()
// }

// #[test]
// fn disable_load_and_get_reading() {
//     todo!()
// }

// #[test]
// fn enable_generator_and_get_reading() {
//     todo!()
// }

#[test]
fn tap_regulator_up_and_get_reading() {
    dss::start(0);
    dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss");
    assert!(circuit::set_active_element("regcontrol.Reg1").is_ok());

    let cur_tap = reg_controls::get_tap_number();
    reg_controls::set_tap_number(cur_tap + 1);

    assert!(circuit::set_active_element("Transformer.Reg1").is_ok());

    let readings = ckt_element::get_voltages_mag_ang();
    assert!(readings.is_ok());
}

#[test]
fn tap_regulator_down_and_get_reading() {
    dss::start(0);
    dss::text_set_command("redirect tests/data/IEEE13Nodeckt.dss");
    assert!(circuit::set_active_element("regcontrol.Reg1").is_ok());

    let cur_tap = reg_controls::get_tap_number();
    reg_controls::set_tap_number(cur_tap - 1);

    assert!(circuit::set_active_element("Transformer.Reg1").is_ok());
    let readings = ckt_element::get_voltages_mag_ang();
    assert!(readings.is_ok());
}

// #[test]
// fn open_recloser_and_get_reading() {
//     todo!()
// }

// #[test]
// fn close_recloser_and_get_reading() {
//     todo!()
// }

// #[test]
// fn open_switch_and_get_reading() {
//     todo!()
// }

// #[test]
// fn close_switch_and_get_reading() {
//     todo!()
// }

// #[test]
// fn set_generator_and_get_reading() {
//     todo!()
// }

// #[test]
// fn set_load_and_get_reading() {
//     todo!()
// }
