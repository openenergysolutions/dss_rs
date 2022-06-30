extern "C" {
    pub fn Capacitors_Get_AllNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Capacitors_Get_AllNames_GR();
}
extern "C" {
    pub fn Capacitors_Get_First() -> i32;
}
extern "C" {
    pub fn Capacitors_Get_IsDelta() -> u16;
}
extern "C" {
    pub fn Capacitors_Get_kV() -> f64;
}
extern "C" {
    pub fn Capacitors_Get_kvar() -> f64;
}
extern "C" {
    pub fn Capacitors_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Capacitors_Get_Next() -> i32;
}
extern "C" {
    pub fn Capacitors_Get_NumSteps() -> i32;
}
extern "C" {
    pub fn Capacitors_Set_IsDelta(Value: u16);
}
extern "C" {
    pub fn Capacitors_Set_kV(Value: f64);
}
extern "C" {
    pub fn Capacitors_Set_kvar(Value: f64);
}
extern "C" {
    pub fn Capacitors_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Capacitors_Set_NumSteps(Value: i32);
}
extern "C" {
    pub fn Capacitors_Get_Count() -> i32;
}
extern "C" {
    pub fn Capacitors_AddStep() -> u16;
}
extern "C" {
    pub fn Capacitors_SubtractStep() -> u16;
}
extern "C" {
    pub fn Capacitors_Get_AvailableSteps() -> i32;
}
extern "C" {
    pub fn Capacitors_Get_States(ResultPtr: *mut *mut i32, ResultCount: *mut i32);
}
extern "C" {
    pub fn Capacitors_Get_States_GR();
}
extern "C" {
    pub fn Capacitors_Set_States(ValuePtr: *mut i32, ValueCount: i32);
}
extern "C" {
    pub fn Capacitors_Open();
}
extern "C" {
    pub fn Capacitors_Close();
}
