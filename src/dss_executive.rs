extern "C" {
    pub fn DSS_Executive_Get_Command(i: i32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Executive_Get_NumCommands() -> i32;
}
extern "C" {
    pub fn DSS_Executive_Get_NumOptions() -> i32;
}
extern "C" {
    pub fn DSS_Executive_Get_Option(i: i32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Executive_Get_CommandHelp(i: i32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Executive_Get_OptionHelp(i: i32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Executive_Get_OptionValue(i: i32) -> *mut ::std::os::raw::c_char;
}
