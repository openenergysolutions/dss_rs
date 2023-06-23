extern crate dss_rs_sys;
use dss_rs_sys as dss_c;
use std::convert::TryInto;

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
