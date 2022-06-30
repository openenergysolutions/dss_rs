extern "C" {
    pub fn Transformers_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Transformers_Get_AllNames_GR();
}
extern "C" {
    pub fn Transformers_Get_First() -> i32;
}
extern "C" {
    pub fn Transformers_Get_IsDelta() -> u16;
}
extern "C" {
    pub fn Transformers_Get_kV() -> f64;
}
extern "C" {
    pub fn Transformers_Get_kVA() -> f64;
}
extern "C" {
    pub fn Transformers_Get_MaxTap() -> f64;
}
extern "C" {
    pub fn Transformers_Get_MinTap() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Transformers_Get_Next() -> i32;
}
extern "C" {
    pub fn Transformers_Get_NumTaps() -> i32;
}
extern "C" {
    pub fn Transformers_Get_NumWindings() -> i32;
}
extern "C" {
    pub fn Transformers_Get_R() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Rneut() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Tap() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Wdg() -> i32;
}
extern "C" {
    pub fn Transformers_Get_XfmrCode() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Transformers_Get_Xhl() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Xht() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Xlt() -> f64;
}
extern "C" {
    pub fn Transformers_Get_Xneut() -> f64;
}
extern "C" {
    pub fn Transformers_Set_IsDelta(Value: u16);
}
extern "C" {
    pub fn Transformers_Set_kV(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_kVA(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_MaxTap(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_MinTap(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Transformers_Set_NumTaps(Value: i32);
}
extern "C" {
    pub fn Transformers_Set_NumWindings(Value: i32);
}
extern "C" {
    pub fn Transformers_Set_R(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Rneut(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Tap(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Wdg(Value: i32);
}
extern "C" {
    pub fn Transformers_Set_XfmrCode(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Transformers_Set_Xhl(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Xht(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Xlt(Value: f64);
}
extern "C" {
    pub fn Transformers_Set_Xneut(Value: f64);
}
extern "C" {
    pub fn Transformers_Get_Count() -> i32;
}
extern "C" {
    pub fn Transformers_Get_WdgVoltages(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Transformers_Get_WdgVoltages_GR();
}
extern "C" {
    pub fn Transformers_Get_WdgCurrents(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Transformers_Get_WdgCurrents_GR();
}
extern "C" {
    pub fn Transformers_Get_strWdgCurrents() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Transformers_Get_CoreType() -> i32;
}
extern "C" {
    pub fn Transformers_Set_CoreType(Value: i32);
}
extern "C" {
    pub fn Transformers_Get_RdcOhms() -> f64;
}
extern "C" {
    pub fn Transformers_Set_RdcOhms(Value: f64);
}
extern "C" {
    pub fn Transformers_Get_LossesByType(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Transformers_Get_LossesByType_GR();
}
extern "C" {
    pub fn Transformers_Get_AllLossesByType(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Transformers_Get_AllLossesByType_GR();
}
