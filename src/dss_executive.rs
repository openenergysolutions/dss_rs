use dss_rs_sys as dss_c;

pub unsafe fn get_command(i: i32) -> *mut ::std::os::raw::c_char {
    dss_c::DSS_Executive_Get_Command(i)
}

pub unsafe fn get_num_commands() -> i32 {
    dss_c::DSS_Executive_Get_NumCommands()
}

pub unsafe fn get_num_options() -> i32 {
    dss_c::DSS_Executive_Get_NumOptions()
}

pub unsafe fn get_option(i: i32) -> *mut ::std::os::raw::c_char {
    dss_c::DSS_Executive_Get_Option(i)
}

pub unsafe fn get_command_help(i: i32) -> *mut ::std::os::raw::c_char {
    dss_c::DSS_Executive_Get_CommandHelp(i)
}

pub unsafe fn get_option_help(i: i32) -> *mut ::std::os::raw::c_char {
    dss_c::DSS_Executive_Get_OptionHelp(i)
}

pub unsafe fn get_option_value(i: i32) -> *mut ::std::os::raw::c_char {
    dss_c::DSS_Executive_Get_OptionValue(i)
}
