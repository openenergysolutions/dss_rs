// Copyright 2024 Open Energy Solutions Inc.
// 
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
