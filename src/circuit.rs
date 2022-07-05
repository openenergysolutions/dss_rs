#[cfg(feature = "linux_x64")]
use crate::linux_x64::bindings as dss_c;

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

pub unsafe fn get_line_losses(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_LineLosses(result_ptr, result_count);
}

pub unsafe fn get_line_losses_gr() {
    dss_c::Circuit_Get_LineLosses_GR();
}

pub unsafe fn get_losses(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_Losses(result_ptr, result_count);
}

pub unsafe fn get_losses_gr() {
    dss_c::Circuit_Get_Losses_GR();
}

pub unsafe fn get_all_bus_bmag(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllBusVmag(result_ptr, result_count);
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

pub unsafe fn get_total_power(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_TotalPower(result_ptr, result_count);
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

pub unsafe fn get_all_bus_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::Circuit_Get_AllBusNames(result_ptr, result_count);
}

pub unsafe fn get_all_bus_names_gr() {
    dss_c::Circuit_Get_AllBusNames_GR();
}

pub unsafe fn get_all_element_losses(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllElementLosses(result_ptr, result_count);
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

pub unsafe fn set_active_element(full_name: *mut ::std::os::raw::c_char) -> i32 {
    dss_c::Circuit_SetActiveElement(full_name)
}

pub unsafe fn capacity(start: f64, increment: f64) -> f64 {
    dss_c::Circuit_Capacity(start, increment)
}

pub unsafe fn get_all_bus_vmag_pu(result_ptr: *mut *mut f64, result_count: *mut i32) {
    dss_c::Circuit_Get_AllBusVmagPu(result_ptr, result_count);
}

pub unsafe fn get_all_bus_vmag_pu_gr() {
    dss_c::Circuit_Get_AllBusVmagPu_GR();
}

pub unsafe fn set_active_bus(bus_name: *mut ::std::os::raw::c_char) -> i32 {
    dss_c::Circuit_SetActiveBus(bus_name)
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
