extern crate dss_rs_sys;
use dss_rs_sys as dss_c;
use crate::{ckt_element, dss, dss_result::{Result, DssError}};
use std::{ptr, convert::TryInto};

pub fn get_pu_soc() -> f64 {
    unsafe { dss_c::Storages_Get_puSOC() }
}

pub fn set_pu_soc(value: f64) {
    unsafe {
        dss_c::Storages_Set_puSOC(value);
    }
}

pub fn get_state() -> i32 {
    unsafe { dss_c::Storages_Get_State() }
}

pub fn set_state(value: i32) {
    unsafe {
        dss_c::Storages_Set_State(value);
    }
}

pub fn set_kw(value: f64) -> Result<()> {
    unsafe {
        let ctx = dss_c::ctx_Get_Prime();
        if ctx == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
    
        let element_idx = dss_c::Storages_Get_idx();
        let class_idx = dss::set_active_class("Storage")?;

        let element_ptr = dss_c::Obj_GetHandleByIdx(ctx, class_idx, element_idx);

        if element_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }

        let property_idx = (ckt_element::get_all_property_names().iter().position(|p| p == "kW").unwrap() + 1) as i32;
        dss_c::Obj_SetFloat64(element_ptr, property_idx, value);
    }
    Ok(())
}

pub fn get_kw() -> Result<f64> {
    unsafe {
        let ctx = dss_c::ctx_Get_Prime();
        if ctx == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }

        let element_idx = dss_c::Storages_Get_idx();
        let class_idx = dss::set_active_class("Storage")?;

        let element_ptr = dss_c::Obj_GetHandleByIdx(ctx, class_idx, element_idx);
        if element_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }

        let property_names = ckt_element::get_all_property_names();
        let property_idx = (property_names.iter().position(|p| p == "kW").unwrap() + 1) as i32;
        let kw = dss_c::Obj_GetFloat64(element_ptr, property_idx.try_into().unwrap());

        Ok(kw)
    }
}