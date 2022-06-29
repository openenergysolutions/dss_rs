pub fn get_Name() -> *mut ::std::os::raw::c_char {}

pub fn get_NumNodes() -> i32 {}

pub fn get_SeqVoltages(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_SeqVoltages_Gr() {}

pub fn get_Voltages(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_Voltages_Gr() {}

pub fn get_Nodes(result_ptr: *mut *mut i32, result_count: *mut i32) {}

pub fn get_Nodes_Gr() {}

pub fn get_Isc(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_Isc_Gr() {}

pub fn get_Voc(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_Voc_Gr() {}

pub fn get_kVBase() -> f64 {}

pub fn get_puVoltages(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_puVoltages_Gr() {}

pub fn get_Zsc0(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_Zsc0_Gr() {}

pub fn get_Zsc1(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_Zsc1_Gr() {}

pub fn get_ZscMatrix(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_ZscMatrix_Gr() {}

pub fn Zscrefresh() -> u16 {}

pub fn get_YscMatrix(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_YscMatrix_Gr() {}

pub fn get__coorddefined() -> u16 {}

pub fn get_x() -> f64 {}

pub fn Set_x(Value: f64) {}

pub fn get_y() -> f64 {}

pub fn Set_y(Value: f64) {}

pub fn get_Distance() -> f64 {}

pub fn getUniqueNodeNumber(StartNumber: i32) -> i32 {}

pub fn get__cplxSeqVoltages(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get__cplxSeqVoltages_Gr() {}

pub fn get_Int_Duration() -> f64 {}

pub fn get_Lambda() -> f64 {}

pub fn get__cust_Duration() -> f64 {}

pub fn get__cust_Interrupts() -> f64 {}

pub fn get_N__customers() -> i32 {}

pub fn get_N_interrupts() -> f64 {}

pub fn get_puVLL(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_puVLL_Gr() {}

pub fn get_VLL(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_VLL_Gr() {}

pub fn get_puVmagAngle(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_puVmagAngle_Gr() {}

pub fn get_VMagAngle(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_VMagAngle_Gr() {}

pub fn get_TotalMiles() -> f64 {}

pub fn get_SectionID() -> i32 {}

pub fn get_LineList(result_ptr: *mut *mut *mut ::std::os::raw::c_char, result_count: *mut i32) {}

pub fn get_LineList_Gr() {}

pub fn get_load_list(result_ptr: *mut *mut *mut ::std::os::raw::c_char, result_count: *mut i32) {}

pub fn get_load_list_gr() {}

pub fn get_zs_c012_matrix(result_ptr: *mut *mut f64, result_count: *mut i32) {}

pub fn get_zs_c012_matrix_gr() {}

pub fn get_all_pce_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
}

pub fn get_all_pde_at_bus(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
}
