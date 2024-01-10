// Copyright 2024 Open Energy Solutions Inc.
// 
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::dss_result::{DssError, Result};
use dss_rs_sys as dss_c;
use std::{ffi::CStr, os::raw::c_char, ptr, slice};

pub fn get_name() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::Bus_Get_Name();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn get_num_nodes() -> i32 {
    unsafe { dss_c::Bus_Get_NumNodes() }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_seq_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_SeqVoltages(result_ptr, result_count);
}

pub fn get_seq_voltages_gr() {
    unsafe {
        dss_c::Bus_Get_SeqVoltages_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_Voltages(result_ptr, result_count);
}

pub fn get_voltages_gr() {
    unsafe {
        dss_c::Bus_Get_Voltages_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_nodes(result_ptr: *mut *mut i32, result_count: *mut i32) {
    dss_c::Bus_Get_Nodes(result_ptr, result_count);
}

pub fn get_nodes_gr() {
    unsafe {
        dss_c::Bus_Get_Nodes_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_isc(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_Isc(result_ptr, result_count);
}

pub fn get_isc_gr() {
    unsafe {
        dss_c::Bus_Get_Isc_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_voc(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_Voc(result_ptr, result_count);
}

pub fn get_voc_gr() {
    unsafe {
        dss_c::Bus_Get_Voc_GR();
    }
}

pub fn get_kv_base() -> f64 {
    unsafe { dss_c::Bus_Get_kVBase() }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_pu_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_puVoltages(result_ptr, result_count);
}

pub fn get_pu_voltages_gr() {
    unsafe {
        dss_c::Bus_Get_puVoltages_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_zsc0(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_Zsc0(result_ptr, result_count);
}

pub fn get_zsc0_gr() {
    unsafe {
        dss_c::Bus_Get_Zsc0_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_zsc1(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_Zsc1(result_ptr, result_count);
}

pub fn get_zsc1_gr() {
    unsafe {
        dss_c::Bus_Get_Zsc1_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_zsc_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_ZscMatrix(result_ptr, result_count);
}

pub fn get_zsc_matrix_gr() {
    unsafe {
        dss_c::Bus_Get_ZscMatrix_GR();
    }
}

pub fn zscrefresh() -> u16 {
    unsafe { dss_c::Bus_ZscRefresh() }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_ysc_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_YscMatrix(result_ptr, result_count);
}

pub fn get_ysc_matrix_gr() {
    unsafe {
        dss_c::Bus_Get_YscMatrix_GR();
    }
}

pub fn get_coorddefined() -> u16 {
    unsafe { dss_c::Bus_Get_Coorddefined() }
}

pub fn get_x() -> f64 {
    unsafe { dss_c::Bus_Get_x() }
}

pub fn set_x(value: f64) {
    unsafe {
        dss_c::Bus_Set_x(value);
    }
}

pub fn get_y() -> f64 {
    unsafe { dss_c::Bus_Get_y() }
}

pub fn set_y(value: f64) {
    unsafe {
        dss_c::Bus_Set_y(value);
    }
}

pub fn get_distance() -> f64 {
    unsafe { dss_c::Bus_Get_Distance() }
}

pub fn get_unique_node_number(start_number: i32) -> i32 {
    unsafe { dss_c::Bus_GetUniqueNodeNumber(start_number) }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_cplx_seq_voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_CplxSeqVoltages(result_ptr, result_count);
}

pub fn get_cplx_seq_voltages_gr() {
    unsafe {
        dss_c::Bus_Get_CplxSeqVoltages_GR();
    }
}

pub fn get_int_duration() -> f64 {
    unsafe { dss_c::Bus_Get_Int_Duration() }
}

pub fn get_lambda() -> f64 {
    unsafe { dss_c::Bus_Get_Lambda() }
}

pub fn get_cust_duration() -> f64 {
    unsafe { dss_c::Bus_Get_Cust_Duration() }
}

pub fn get_cust_interrupts() -> f64 {
    unsafe { dss_c::Bus_Get_Cust_Interrupts() }
}

pub fn get_n_customers() -> i32 {
    unsafe { dss_c::Bus_Get_N_Customers() }
}

pub fn get_n_interrupts() -> f64 {
    unsafe { dss_c::Bus_Get_N_interrupts() }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_pu_vll(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_puVLL(result_ptr, result_count);
}

pub fn get_pu_vll_gr() {
    unsafe {
        dss_c::Bus_Get_puVLL_GR();
    }
}

pub fn get_vll() -> Result<Vec<f64>> {
    unsafe {
        let mut result_count = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Bus_Get_VLL(&mut result_ptr, &mut result_count);
        if result_count == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_count as usize).to_vec();
        Ok(v)
    }
}

pub fn get_vll_gr() {
    unsafe {
        dss_c::Bus_Get_VLL_GR();
    }
}

pub fn get_pu_vmag_angle() -> Result<Vec<f64>> {
    unsafe {
        let mut result_count = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Bus_Get_puVmagAngle(&mut result_ptr, &mut result_count);
        if result_count == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_count as usize).to_vec();
        Ok(v)
    }
}

pub fn get_pu_vmag_angle_gr() {
    unsafe {
        dss_c::Bus_Get_puVmagAngle_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_vmag_angle(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_VMagAngle(result_ptr, result_count);
}

pub fn get_vmag_angle_gr() {
    unsafe {
        dss_c::Bus_Get_VMagAngle_GR();
    }
}

pub fn get_total_miles() -> f64 {
    unsafe { dss_c::Bus_Get_TotalMiles() }
}

pub fn get_section_id() -> i32 {
    unsafe { dss_c::Bus_Get_SectionID() }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_line_list(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Bus_Get_LineList(result_ptr, result_count);
}

pub fn get_line_list_gr() {
    unsafe {
        dss_c::Bus_Get_LineList_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_load_list(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Bus_Get_LoadList(result_ptr, result_count);
}

pub fn get_load_list_gr() {
    unsafe {
        dss_c::Bus_Get_LoadList_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_zsc_012_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Bus_Get_ZSC012Matrix(result_ptr, result_count);
}

pub fn get_zsc_012_matrix_gr() {
    unsafe {
        dss_c::Bus_Get_ZSC012Matrix_GR();
    }
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_all_pce_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Bus_Get_AllPCEatBus(result_ptr, result_count);
}

#[cfg(feature = "unsafe")]
pub unsafe fn get_all_pde_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Bus_Get_AllPDEatBus(result_ptr, result_count);
}
