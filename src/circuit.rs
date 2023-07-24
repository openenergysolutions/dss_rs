extern crate dss_rs_sys;
use crate::dss_result::{DssError, Result};
use dss_rs_sys as dss_c;
use std::{ffi::CStr, ffi::CString, ptr, slice};

pub unsafe fn get_name() -> *mut ::std::os::raw::c_char {
    dss_c::Circuit_Get_Name()
}

pub unsafe fn get_num_buses() -> i32 {
    dss_c::Circuit_Get_NumBuses()
}

pub unsafe fn get_num_ckt_elements() -> i32 {
    dss_c::Circuit_Get_NumCktElements()
}

pub unsafe fn get_num_nodes() -> i32 {
    dss_c::Circuit_Get_NumNodes()
}

pub fn get_line_losses() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_LineLosses(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_line_losses_gr() {
    dss_c::Circuit_Get_LineLosses_GR();
}

pub fn get_losses() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_Losses(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_losses_gr() {
    dss_c::Circuit_Get_Losses_GR();
}

pub fn get_all_bus_vmag() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_AllBusVmag(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_all_bus_vmag_gr() {
    dss_c::Circuit_Get_AllBusVmag_GR();
}

pub unsafe fn get_all_bus_volts(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllBusVolts(result_ptr, result_count);
}

pub unsafe fn get_all_bus_volts_gr() {
    dss_c::Circuit_Get_AllBusVolts_GR();
}

pub unsafe fn get_all_element_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Circuit_Get_AllElementNames(result_ptr, result_count);
}

pub unsafe fn get_all_element_names_gr() {
    dss_c::Circuit_Get_AllElementNames_GR();
}

pub unsafe fn get_substation_losses(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_SubstationLosses(result_ptr, result_count);
}

pub unsafe fn get_substation_losses_gr() {
    dss_c::Circuit_Get_SubstationLosses_GR();
}

pub fn get_total_power() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_Losses(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_total_power_gr() {
    dss_c::Circuit_Get_TotalPower_GR();
}

pub unsafe fn disable(name: *mut ::std::os::raw::c_char) {
    dss_c::Circuit_Disable(name);
}

pub unsafe fn enable(name: *mut ::std::os::raw::c_char) {
    dss_c::Circuit_Enable(name);
}

pub unsafe fn first_pc_element() -> i32 {
    dss_c::Circuit_FirstPCElement()
}

pub unsafe fn first_pd_elementt() -> i32 {
    dss_c::Circuit_FirstPDElement()
}
pub unsafe fn next_pc_element() -> i32 {
    dss_c::Circuit_NextPCElement()
}

pub unsafe fn next_pd_element() -> i32 {
    dss_c::Circuit_NextPDElement()
}

pub fn get_all_bus_names() -> Result<Vec<String>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_AllBusNames(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();

        let vec: Vec<String> = v
            .iter()
            .filter_map(|p| CStr::from_ptr(*p).to_str().ok())
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect();
        Ok(vec)
    }
}

pub unsafe fn get_all_bus_names_gr() {
    dss_c::Circuit_Get_AllBusNames_GR();
}

pub fn get_all_element_losses() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_AllElementLosses(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_all_element_losses_gr() {
    dss_c::Circuit_Get_AllElementLosses_GR();
}

pub unsafe fn sample() {
    dss_c::Circuit_Sample();
}

pub unsafe fn save_sample() {
    dss_c::Circuit_SaveSample();
}

pub fn set_active_element(full_name: &str) -> Result<i32> {
    unsafe {
        let c_str = CString::new(full_name)?;
        let ret = dss_c::Circuit_SetActiveElement(c_str.into_raw());
        if ret < 0 {
            return Err(DssError::CallFail);
        }
        Ok(ret)
    }
}

pub unsafe fn capacity(start: f64, increment: f64) -> f64 {
    dss_c::Circuit_Capacity(start, increment)
}

pub fn get_all_bus_vmag_pu() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::Circuit_Get_AllBusVmagPu(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub unsafe fn get_all_bus_vmag_pu_gr() {
    dss_c::Circuit_Get_AllBusVmagPu_GR();
}

pub fn set_active_bus(value: &str) -> Result<i32> {
    unsafe {
        let c_str = CString::new(value)?;
        Ok(dss_c::Circuit_SetActiveBus(c_str.into_raw()))
    }
}

pub unsafe fn set_active_bus_i(bus_index: i32) -> i32 {
    dss_c::Circuit_SetActiveBusi(bus_index)
}

pub unsafe fn get_all_node_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Circuit_Get_AllNodeNames(result_ptr, result_count);
}

pub unsafe fn get_all_node_names_gr() {
    dss_c::Circuit_Get_AllNodeNames_GR();
}

pub unsafe fn get_system_y(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_SystemY(result_ptr, result_count);
}

pub unsafe fn get_system_y_gr() {
    dss_c::Circuit_Get_SystemY_GR();
}

pub unsafe fn get_all_bus_distances(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllBusDistances(result_ptr, result_count);
}

pub unsafe fn get_all_bus_distances_gr() {
    dss_c::Circuit_Get_AllBusDistances_GR();
}

pub unsafe fn get_all_node_distances(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllNodeDistances(result_ptr, result_count);
}

pub unsafe fn get_all_node_distances_gr() {
    dss_c::Circuit_Get_AllNodeDistances_GR();
}

pub unsafe fn get_all_node_distances_by_phase(
    result_ptr: *mut *mut f64,
    result_count: *mut i32,
    phase: i32,
) {
    dss_c::Circuit_Get_AllNodeDistancesByPhase(result_ptr, result_count, phase);
}

pub unsafe fn get_all_node_distances_by_phase_gr(phase: i32) {
    dss_c::Circuit_Get_AllNodeDistancesByPhase_GR(phase);
}

pub unsafe fn get_all_node_vmag_by_phase(
    result_ptr: *mut *mut f64,
    result_count: *mut i32,
    phase: i32,
) {
    dss_c::Circuit_Get_AllNodeVmagByPhase(result_ptr, result_count, phase);
}

pub unsafe fn get_all_node_vmag_by_phase_gr(phase: i32) {
    dss_c::Circuit_Get_AllNodeVmagByPhase_GR(phase);
}

pub unsafe fn get_all_node_vmag_pu_by_phase(
    result_ptr: *mut *mut f64,
    result_count: *mut i32,
    phase: i32,
) {
    dss_c::Circuit_Get_AllNodeVmagPUByPhase(result_ptr, result_count, phase);
}

pub unsafe fn get_all_node_vmag_pu_by_phase_gr(phase: i32) {
    dss_c::Circuit_Get_AllNodeVmagPUByPhase_GR(phase);
}

pub unsafe fn get_all_node_names_by_phase(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
    phase: i32,
) {
    dss_c::Circuit_Get_AllNodeNamesByPhase(result_ptr, result_count, phase);
}

pub unsafe fn get_all_node_names_by_phase_gr(phase: i32) {
    dss_c::Circuit_Get_AllNodeNamesByPhase_GR(phase);
}

pub unsafe fn set_active_class(class_name: *mut ::std::os::raw::c_char) -> i32 {
    dss_c::Circuit_SetActiveClass(class_name)
}

pub unsafe fn first_element() -> i32 {
    dss_c::Circuit_FirstElement()
}

pub unsafe fn next_element() -> i32 {
    dss_c::Circuit_NextElement()
}

pub unsafe fn update_storage() {
    dss_c::Circuit_UpdateStorage();
}

pub unsafe fn get_parent_pd_element() -> i32 {
    dss_c::Circuit_Get_ParentPDElement()
}

pub unsafe fn end_of_timestep_update() {
    dss_c::Circuit_EndOfTimeStepUpdate();
}

pub unsafe fn get_y_node_order(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Circuit_Get_YNodeOrder(result_ptr, result_count);
}

pub unsafe fn get_y_node_order_gr() {
    dss_c::Circuit_Get_YNodeOrder_GR();
}

pub unsafe fn get_y_currents(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_YCurrents(result_ptr, result_count);
}

pub unsafe fn get_y_currents_gr() {
    dss_c::Circuit_Get_YCurrents_GR();
}

pub unsafe fn get_y_node_v_array(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_YNodeVarray(result_ptr, result_count);
}

pub unsafe fn get_y_node_varray_gr() {
    dss_c::Circuit_Get_YNodeVarray_GR();
}

pub unsafe fn set_ckt_element_mame(value: *mut ::std::os::raw::c_char) {
    dss_c::Circuit_SetCktElementName(value);
}

pub unsafe fn set_ckt_element_index(value: i32) {
    dss_c::Circuit_SetCktElementIndex(value);
}
