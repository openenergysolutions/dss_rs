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

pub unsafe fn get_all_names(
    result_ptr: *mut *mut *mut ::std::os::raw::c_char,
    result_count: *mut i32,
) {
    dss_c::CapControls_Get_AllNames(result_ptr, result_count);
}

pub unsafe fn get_all_names_gr() {
    dss_c::CapControls_Get_AllNames_GR();
}

pub unsafe fn get_capacitor() -> *mut ::std::os::raw::c_char {
    dss_c::CapControls_Get_Capacitor()
}

pub unsafe fn get_ct_ratio() -> f64 {
    dss_c::CapControls_Get_CTratio()
}

pub unsafe fn get_dead_time() -> f64 {
    dss_c::CapControls_Get_DeadTime()
}

pub unsafe fn get_delay() -> f64 {
    dss_c::CapControls_Get_Delay()
}

pub unsafe fn get_delay_off() -> f64 {
    dss_c::CapControls_Get_DelayOff()
}

pub unsafe fn get_first() -> i32 {
    dss_c::CapControls_Get_First()
}

pub unsafe fn get_mode() -> i32 {
    dss_c::CapControls_Get_Mode()
}

pub unsafe fn get_monitored_obj() -> *mut ::std::os::raw::c_char {
    dss_c::CapControls_Get_MonitoredObj()
}

pub unsafe fn get_monitored_term() -> i32 {
    dss_c::CapControls_Get_MonitoredTerm()
}

pub unsafe fn get_name() -> *mut ::std::os::raw::c_char {
    dss_c::CapControls_Get_Name()
}

pub unsafe fn get_next() -> i32 {
    dss_c::CapControls_Get_Next()
}

pub unsafe fn get_off_setting() -> f64 {
    dss_c::CapControls_Get_OFFSetting()
}

pub unsafe fn get_on_setting() -> f64 {
    dss_c::CapControls_Get_ONSetting()
}

pub unsafe fn get_pt_ratio() -> f64 {
    dss_c::CapControls_Get_PTratio()
}

pub unsafe fn get_use_volt_override() -> u16 {
    dss_c::CapControls_Get_UseVoltOverride()
}

pub unsafe fn get_vmax() -> f64 {
    dss_c::CapControls_Get_Vmax()
}

pub unsafe fn get_vmin() -> f64 {
    dss_c::CapControls_Get_Vmin()
}

pub unsafe fn set_capacitor(value: *mut ::std::os::raw::c_char) {
    dss_c::CapControls_Set_Capacitor(value);
}

pub unsafe fn set_ct_ratio(value: f64) {
    dss_c::CapControls_Set_CTratio(value);
}

pub unsafe fn set_dead_time(value: f64) {
    dss_c::CapControls_Set_DeadTime(value);
}

pub unsafe fn set_delay(value: f64) {
    dss_c::CapControls_Set_Delay(value);
}

pub unsafe fn set_delay_off(value: f64) {
    dss_c::CapControls_Set_DelayOff(value);
}

pub unsafe fn set_mode(value: i32) {
    dss_c::CapControls_Set_Mode(value);
}

pub unsafe fn set_monitored_obj(value: *mut ::std::os::raw::c_char) {
    dss_c::CapControls_Set_MonitoredObj(value);
}

pub unsafe fn set_monitored_term(value: i32) {
    dss_c::CapControls_Set_MonitoredTerm(value);
}

pub unsafe fn set_name(value: *mut ::std::os::raw::c_char) {
    dss_c::CapControls_Set_Name(value);
}

pub unsafe fn set_off_setting(value: f64) {
    dss_c::CapControls_Set_OFFSetting(value);
}

pub unsafe fn set_on_setting(value: f64) {
    dss_c::CapControls_Set_ONSetting(value);
}

pub unsafe fn set_pt_ratio(value: f64) {
    dss_c::CapControls_Set_PTratio(value);
}

pub unsafe fn set_use_volt_override(value: u16) {
    dss_c::CapControls_Set_UseVoltOverride(value);
}

pub unsafe fn set_vmax(value: f64) {
    dss_c::CapControls_Set_Vmax(value);
}

pub unsafe fn set_vmin(value: f64) {
    dss_c::CapControls_Set_Vmin(value);
}

pub unsafe fn get_count() -> i32 {
    dss_c::CapControls_Get_Count()
}

pub unsafe fn reset() {
    dss_c::CapControls_Reset();
}
