extern crate dss_rs_sys;
use dss_rs_sys as dss_c;
use crate::dss_result::Result;
use std::{ffi::CStr, os::raw::c_char};
use std::convert::TryInto;

pub fn get_is_locked() -> u16 {
    unsafe {
        dss_c::SwtControls_Get_IsLocked()
    }
}

pub fn set_is_locked(value: u16) {
    unsafe {
        dss_c::SwtControls_Set_IsLocked(value);
    }
}

pub fn set_action(value: i32) {
    unsafe {
        dss_c::SwtControls_Set_Action(value);
    }
}

pub fn get_switched_obj() -> Result<String> {
    unsafe {
        let raw_ptr: *const c_char = &*dss_c::SwtControls_Get_SwitchedObj();
        let raw_str = CStr::from_ptr(raw_ptr);
        Ok(String::from(raw_str.to_str()?))
    }
}

pub fn get_switched_term() -> i32 {
    unsafe {
        dss_c::SwtControls_Get_SwitchedTerm()
    }
}

pub fn get_state() -> i32 {
    unsafe {
        dss_c::SwtControls_Get_State()

    }
}

pub fn set_state(value: i32) {
    unsafe {
        dss_c::SwtControls_Set_State(value);
    }
}

pub fn open() {
    unsafe {
        let open = dss_c::ActionCodes_ActionCodes_Open.try_into();
        if let Err(e) = open {
            panic!("{:?}", e);
        }
        dss_c::SwtControls_Set_Action(open.unwrap());
    }
}

pub fn close() {
    unsafe {
        let close = dss_c::ActionCodes_ActionCodes_Close.try_into();
        if let Err(e) = close {
            panic!("{:?}", e);
        }
        dss_c::SwtControls_Set_Action(close.unwrap());
    }
}

pub fn reset() {
    unsafe {
        dss_c::SwtControls_Reset();
    }
}
