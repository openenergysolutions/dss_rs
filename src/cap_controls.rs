extern "C" {
    pub fn CapControls_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn CapControls_Get_AllNames_GR();
}
extern "C" {
    pub fn CapControls_Get_Capacitor() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn CapControls_Get_CTratio() -> f64;
}
extern "C" {
    pub fn CapControls_Get_DeadTime() -> f64;
}
extern "C" {
    pub fn CapControls_Get_Delay() -> f64;
}
extern "C" {
    pub fn CapControls_Get_DelayOff() -> f64;
}
extern "C" {
    pub fn CapControls_Get_First() -> i32;
}
extern "C" {
    pub fn CapControls_Get_Mode() -> i32;
}
extern "C" {
    pub fn CapControls_Get_MonitoredObj() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn CapControls_Get_MonitoredTerm() -> i32;
}
extern "C" {
    pub fn CapControls_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn CapControls_Get_Next() -> i32;
}
extern "C" {
    pub fn CapControls_Get_OFFSetting() -> f64;
}
extern "C" {
    pub fn CapControls_Get_ONSetting() -> f64;
}
extern "C" {
    pub fn CapControls_Get_PTratio() -> f64;
}
extern "C" {
    pub fn CapControls_Get_UseVoltOverride() -> u16;
}
extern "C" {
    pub fn CapControls_Get_Vmax() -> f64;
}
extern "C" {
    pub fn CapControls_Get_Vmin() -> f64;
}
extern "C" {
    pub fn CapControls_Set_Capacitor(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn CapControls_Set_CTratio(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_DeadTime(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_Delay(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_DelayOff(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_Mode(Value: i32);
}
extern "C" {
    pub fn CapControls_Set_MonitoredObj(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn CapControls_Set_MonitoredTerm(Value: i32);
}
extern "C" {
    pub fn CapControls_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn CapControls_Set_OFFSetting(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_ONSetting(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_PTratio(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_UseVoltOverride(Value: u16);
}
extern "C" {
    pub fn CapControls_Set_Vmax(Value: f64);
}
extern "C" {
    pub fn CapControls_Set_Vmin(Value: f64);
}
extern "C" {
    pub fn CapControls_Get_Count() -> i32;
}
extern "C" {
    pub fn CapControls_Reset();
}
