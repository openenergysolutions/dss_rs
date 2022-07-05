#[cfg(feature = "linux_x64")]
use crate::linux_x64::bindings as dss_c;

pub unsafe fn get_all_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Capacitors_Get_AllNames(result_ptr, result_count);
}

pub unsafe fn get_all_names_gr() {
    dss_c::Capacitors_Get_AllNames_GR();
}

pub unsafe fn get_first() -> i32 {
    dss_c::Capacitors_Get_First()
}

pub unsafe fn get_is_delta() -> u16 {
    dss_c::Capacitors_Get_IsDelta()
}

pub unsafe fn get_kv() -> f64 {
    dss_c::Capacitors_Get_kV()
}

pub unsafe fn get_kvar() -> f64 {
    dss_c::Capacitors_Get_kvar()
}

pub unsafe fn get_name() -> *mut ::std::os::raw::c_char {
    dss_c::Capacitors_Get_Name()
}

pub unsafe fn get_next() -> i32 {
    dss_c::Capacitors_Get_Next()
}

pub unsafe fn get_num_steps() -> i32 {
    dss_c::Capacitors_Get_NumSteps()
}

pub unsafe fn set_is_delta(value: u16) {
    dss_c::Capacitors_Set_IsDelta(value);
}

pub unsafe fn set_kv(value: f64) {
    dss_c::Capacitors_Set_kV(value);
}

pub unsafe fn set_kvar(value: f64) {
    dss_c::Capacitors_Set_kvar(value);
}

pub unsafe fn set_name(value: *mut ::std::os::raw::c_char) {
    dss_c::Capacitors_Set_Name(value);
}

pub unsafe fn set_num_steps(value: i32) {
    dss_c::Capacitors_Set_NumSteps(value);
}

pub unsafe fn get_count() -> i32 {
    dss_c::Capacitors_Get_Count()
}

pub unsafe fn add_step() -> u16 {
    dss_c::Capacitors_AddStep()
}

pub unsafe fn subtract_step() -> u16 {
    dss_c::Capacitors_SubtractStep()
}

pub unsafe fn get_available_steps() -> i32 {
    dss_c::Capacitors_Get_AvailableSteps()
}

pub unsafe fn get_states(result_ptr: *mut *mut i32, result_count: *mut i32) {
    dss_c::Capacitors_Get_States(result_ptr, result_count);
}

pub unsafe fn get_states_GR() {
    dss_c::Capacitors_Get_States_GR();
}

pub unsafe fn set_states(value_ptr: *mut i32, value_count: i32) {
    dss_c::Capacitors_Set_States(value_ptr, value_count);
}

pub unsafe fn open() {
    dss_c::Capacitors_Open();
}

pub unsafe fn close() {
    dss_c::Capacitors_Close();
}
