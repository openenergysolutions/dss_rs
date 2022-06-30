extern "C" {
    pub fn Loads_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Loads_Get_AllNames_GR();
}
extern "C" {
    pub fn Loads_Get_First() -> i32;
}
extern "C" {
    pub fn Loads_Get_idx() -> i32;
}
extern "C" {
    pub fn Loads_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_Next() -> i32;
}
extern "C" {
    pub fn Loads_Set_idx(Value: i32);
}
extern "C" {
    pub fn Loads_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Get_kV() -> f64;
}
extern "C" {
    pub fn Loads_Get_kvar() -> f64;
}
extern "C" {
    pub fn Loads_Get_kW() -> f64;
}
extern "C" {
    pub fn Loads_Get_PF() -> f64;
}
extern "C" {
    pub fn Loads_Set_kV(Value: f64);
}
extern "C" {
    pub fn Loads_Set_kvar(Value: f64);
}
extern "C" {
    pub fn Loads_Set_kW(Value: f64);
}
extern "C" {
    pub fn Loads_Set_PF(Value: f64);
}
extern "C" {
    pub fn Loads_Get_Count() -> i32;
}
extern "C" {
    pub fn Loads_Get_AllocationFactor() -> f64;
}
extern "C" {
    pub fn Loads_Get_Cfactor() -> f64;
}
extern "C" {
    pub fn Loads_Get_Class_() -> i32;
}
extern "C" {
    pub fn Loads_Get_CVRcurve() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_CVRvars() -> f64;
}
extern "C" {
    pub fn Loads_Get_CVRwatts() -> f64;
}
extern "C" {
    pub fn Loads_Get_daily() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_duty() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_Growth() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_IsDelta() -> u16;
}
extern "C" {
    pub fn Loads_Get_kva() -> f64;
}
extern "C" {
    pub fn Loads_Get_kwh() -> f64;
}
extern "C" {
    pub fn Loads_Get_kwhdays() -> f64;
}
extern "C" {
    pub fn Loads_Get_Model() -> i32;
}
extern "C" {
    pub fn Loads_Get_NumCust() -> i32;
}
extern "C" {
    pub fn Loads_Get_PctMean() -> f64;
}
extern "C" {
    pub fn Loads_Get_PctStdDev() -> f64;
}
extern "C" {
    pub fn Loads_Get_Rneut() -> f64;
}
extern "C" {
    pub fn Loads_Get_Spectrum() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Get_Status() -> i32;
}
extern "C" {
    pub fn Loads_Get_Vmaxpu() -> f64;
}
extern "C" {
    pub fn Loads_Get_Vminemerg() -> f64;
}
extern "C" {
    pub fn Loads_Get_Vminnorm() -> f64;
}
extern "C" {
    pub fn Loads_Get_Vminpu() -> f64;
}
extern "C" {
    pub fn Loads_Get_xfkVA() -> f64;
}
extern "C" {
    pub fn Loads_Get_Xneut() -> f64;
}
extern "C" {
    pub fn Loads_Get_Yearly() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Loads_Set_AllocationFactor(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Cfactor(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Class_(Value: i32);
}
extern "C" {
    pub fn Loads_Set_CVRcurve(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Set_CVRvars(Value: f64);
}
extern "C" {
    pub fn Loads_Set_CVRwatts(Value: f64);
}
extern "C" {
    pub fn Loads_Set_daily(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Set_duty(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Set_Growth(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Set_IsDelta(Value: u16);
}
extern "C" {
    pub fn Loads_Set_kva(Value: f64);
}
extern "C" {
    pub fn Loads_Set_kwh(Value: f64);
}
extern "C" {
    pub fn Loads_Set_kwhdays(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Model(Value: i32);
}
extern "C" {
    pub fn Loads_Set_NumCust(Value: i32);
}
extern "C" {
    pub fn Loads_Set_PctMean(Value: f64);
}
extern "C" {
    pub fn Loads_Set_PctStdDev(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Rneut(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Spectrum(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Set_Status(Value: i32);
}
extern "C" {
    pub fn Loads_Set_Vmaxpu(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Vminemerg(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Vminnorm(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Vminpu(Value: f64);
}
extern "C" {
    pub fn Loads_Set_xfkVA(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Xneut(Value: f64);
}
extern "C" {
    pub fn Loads_Set_Yearly(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Loads_Get_ZIPV(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Loads_Get_ZIPV_GR();
}
extern "C" {
    pub fn Loads_Set_ZIPV(ValuePtr: *mut f64, ValueCount: i32);
}
extern "C" {
    pub fn Loads_Get_pctSeriesRL() -> f64;
}
extern "C" {
    pub fn Loads_Set_pctSeriesRL(Value: f64);
}
extern "C" {
    pub fn Loads_Get_RelWeight() -> f64;
}
extern "C" {
    pub fn Loads_Set_RelWeight(Value: f64);
}
extern "C" {
    pub fn Loads_Get_Sensor() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn LoadShapes_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn LoadShapes_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn LoadShapes_Get_Count() -> i32;
}
extern "C" {
    pub fn LoadShapes_Get_First() -> i32;
}
extern "C" {
    pub fn LoadShapes_Get_Next() -> i32;
}
extern "C" {
    pub fn LoadShapes_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn LoadShapes_Get_AllNames_GR();
}
extern "C" {
    pub fn LoadShapes_Get_Npts() -> i32;
}
extern "C" {
    pub fn LoadShapes_Get_Pmult(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn LoadShapes_Get_Pmult_GR();
}
extern "C" {
    pub fn LoadShapes_Get_Qmult(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn LoadShapes_Get_Qmult_GR();
}
extern "C" {
    pub fn LoadShapes_Set_Npts(Value: i32);
}
extern "C" {
    pub fn LoadShapes_Set_Pmult(ValuePtr: *mut f64, ValueCount: i32);
}
extern "C" {
    pub fn LoadShapes_Set_Qmult(ValuePtr: *mut f64, ValueCount: i32);
}
extern "C" {
    pub fn LoadShapes_Normalize();
}
extern "C" {
    pub fn LoadShapes_Get_TimeArray(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn LoadShapes_Get_TimeArray_GR();
}
extern "C" {
    pub fn LoadShapes_Set_TimeArray(ValuePtr: *mut f64, ValueCount: i32);
}
extern "C" {
    pub fn LoadShapes_Get_HrInterval() -> f64;
}
extern "C" {
    pub fn LoadShapes_Get_MinInterval() -> f64;
}
extern "C" {
    pub fn LoadShapes_Get_SInterval() -> f64;
}
extern "C" {
    pub fn LoadShapes_Set_SInterval(Value: f64);
}
extern "C" {
    pub fn LoadShapes_Set_HrInterval(Value: f64);
}
extern "C" {
    pub fn LoadShapes_Set_MinInterval(Value: f64);
}
extern "C" {
    pub fn LoadShapes_New(Name: *mut ::std::os::raw::c_char) -> i32;
}
extern "C" {
    pub fn LoadShapes_Get_PBase() -> f64;
}
extern "C" {
    pub fn LoadShapes_Get_Qbase() -> f64;
}
extern "C" {
    pub fn LoadShapes_Set_PBase(Value: f64);
}
extern "C" {
    pub fn LoadShapes_Set_Qbase(Value: f64);
}
extern "C" {
    pub fn LoadShapes_Get_UseActual() -> u16;
}
extern "C" {
    pub fn LoadShapes_Set_UseActual(Value: u16);
}
