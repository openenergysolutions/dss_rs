extern "C" {
    pub fn RegControls_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn RegControls_Get_AllNames_GR();
}
extern "C" {
    pub fn RegControls_Get_CTPrimary() -> f64;
}
extern "C" {
    pub fn RegControls_Get_Delay() -> f64;
}
extern "C" {
    pub fn RegControls_Get_First() -> i32;
}
extern "C" {
    pub fn RegControls_Get_ForwardBand() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ForwardR() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ForwardVreg() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ForwardX() -> f64;
}
extern "C" {
    pub fn RegControls_Get_IsInverseTime() -> u16;
}
extern "C" {
    pub fn RegControls_Get_IsReversible() -> u16;
}
extern "C" {
    pub fn RegControls_Get_MaxTapChange() -> i32;
}
extern "C" {
    pub fn RegControls_Get_MonitoredBus() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn RegControls_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn RegControls_Get_Next() -> i32;
}
extern "C" {
    pub fn RegControls_Get_PTratio() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ReverseBand() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ReverseR() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ReverseVreg() -> f64;
}
extern "C" {
    pub fn RegControls_Get_ReverseX() -> f64;
}
extern "C" {
    pub fn RegControls_Get_TapDelay() -> f64;
}
extern "C" {
    pub fn RegControls_Get_TapWinding() -> i32;
}
extern "C" {
    pub fn RegControls_Get_Transformer() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn RegControls_Get_VoltageLimit() -> f64;
}
extern "C" {
    pub fn RegControls_Get_Winding() -> i32;
}
extern "C" {
    pub fn RegControls_Get_TapNumber() -> i32;
}
extern "C" {
    pub fn RegControls_Set_CTPrimary(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_Delay(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ForwardBand(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ForwardR(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ForwardVreg(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ForwardX(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_IsInverseTime(Value: u16);
}
extern "C" {
    pub fn RegControls_Set_IsReversible(Value: u16);
}
extern "C" {
    pub fn RegControls_Set_MaxTapChange(Value: i32);
}
extern "C" {
    pub fn RegControls_Set_MonitoredBus(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn RegControls_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn RegControls_Set_PTratio(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ReverseBand(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ReverseR(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ReverseVreg(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_ReverseX(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_TapDelay(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_TapWinding(Value: i32);
}
extern "C" {
    pub fn RegControls_Set_Transformer(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn RegControls_Set_VoltageLimit(Value: f64);
}
extern "C" {
    pub fn RegControls_Set_Winding(Value: i32);
}
extern "C" {
    pub fn RegControls_Set_TapNumber(Value: i32);
}
extern "C" {
    pub fn RegControls_Get_Count() -> i32;
}
extern "C" {
    pub fn RegControls_Reset();
}
