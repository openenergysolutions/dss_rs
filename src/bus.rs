#[cfg(feature = "linux_x64")]
use crate::linux_x64 as dss_rs;

pub fn get_name() -> *mut ::std::os::raw::c_char {
    todo!()
}

pub fn get_num_nodes() -> i32 {
    todo!()
}

pub fn get_seq_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_seq_voltages_gr() {
    todo!()
}

pub fn get_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_voltages_gr() {
    todo!()
}

pub fn get_nodes(result_ptr: *mut *mut i32, result_count: *mut i32) {
    todo!()
}

pub fn get_nodes_gr() {
    todo!()
}

pub fn get_isc(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_isc_gr() {
    todo!()
}

pub fn get_voc(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_voc_gr() {
    todo!()
}

pub fn get_kv_base() -> f64 {
    todo!()
}

pub fn get_pu_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_pu_voltages_gr() {
    todo!()
}

pub fn get_zsc0(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_zsc0_gr() {
    todo!()
}

pub fn get_zsc1(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_zsc1_gr() {
    todo!()
}

pub fn get_zsc_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_zsc_matrix_gr() {
    todo!()
}

pub fn zscrefresh() -> u16 {
    todo!()
}

pub fn get_ysc_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_ysc_matrix_gr() {
    todo!()
}

pub fn get_coorddefined() -> u16 {
    todo!()
}

pub fn get_x() -> f64 {
    todo!()
}

pub fn set_x(value: f64) {
    todo!()
}

pub fn get_y() -> f64 {
    todo!()
}

pub fn set_y(value: f64) {
    todo!()
}

pub fn get_distance() -> f64 {
    todo!()
}

pub fn get_unique_node_number(start_number: i32) -> i32 {
    todo!()
}

pub fn get_cplx_seq_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_cplx_seq_voltages_gr() {
    todo!()
}

pub fn get_int_duration() -> f64 {
    todo!()
}

pub fn get_lambda() -> f64 {
    todo!()
}

pub fn get_cust_duration() -> f64 {
    todo!()
}

pub fn get_cust_interrupts() -> f64 {
    todo!()
}

pub fn get_n_customers() -> i32 {
    todo!()
}

pub fn get_n_interrupts() -> f64 {
    todo!()
}

pub fn get_pu_vll(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_pu_vll_gr() {
    todo!()
}

pub fn get_vll(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_vll_gr() {
    todo!()
}

pub fn get_pu_vmag_angle(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_pu_vmag_angle_gr() {
    todo!()
}

pub fn get_vmag_angle(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_vmag_angle_gr() {
    todo!()
}

pub fn get_total_miles() -> f64 {
    todo!()
}

pub fn get_section_id() -> i32 {
    todo!()
}

pub fn get_line_list(result_ptr: *mut *mut *mut ::std::os::raw::c_char, result_count: *mut i32) {
    todo!()
}

pub fn get_line_list_gr() {
    todo!()
}

pub fn get_load_list(result_ptr: *mut *mut *mut ::std::os::raw::c_char, result_count: *mut i32) {
    todo!()
}

pub fn get_load_list_gr() {
    todo!()
}

pub fn get_zs_c012_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    todo!()
}

pub fn get_zs_c012_matrix_gr() {
    todo!()
}

pub fn get_all_pce_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    todo!()
}

pub fn get_all_pde_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    todo!()
}
