extern crate dss_rs_sys;
use crate::{
    ckt_element, dss,
    dss_result::{DssError, Result},
};
use dss_rs_sys as dss_c;
use std::{convert::TryInto, ffi::c_void, ptr};

const KW: &'static str = "kw";
const KVAR: &'static str = "kvar";
const PF: &'static str = "pf";

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
        let element_ptr = get_storage_ptr()?;
        let property_idx = (ckt_element::get_all_property_names()
            .iter()
            .position(|p| p.to_lowercase() == KW)
            .unwrap()
            + 1) as i32;
        dss_c::Obj_SetFloat64(element_ptr, property_idx, value);
        Ok(())
    }
}

pub fn get_kw() -> Result<f64> {
    unsafe {
        let element_ptr = get_storage_ptr()?;
        let property_names = ckt_element::get_all_property_names();
        let property_idx = (property_names
            .iter()
            .position(|p| p.to_lowercase() == KW)
            .unwrap()
            + 1) as i32;
        let kw = dss_c::Obj_GetFloat64(element_ptr, property_idx.try_into().unwrap());
        Ok(kw)
    }
}

pub fn set_kvar(value: f64) -> Result<()> {
    unsafe {
        let element_ptr = get_storage_ptr()?;
        let property_idx = (ckt_element::get_all_property_names()
            .iter()
            .position(|p| p.to_lowercase() == KVAR)
            .unwrap()
            + 1) as i32;
        dss_c::Obj_SetFloat64(element_ptr, property_idx, value);
        Ok(())
    }
}

pub fn get_kvar() -> Result<f64> {
    unsafe {
        let element_ptr = get_storage_ptr()?;
        let property_names = ckt_element::get_all_property_names();
        println!("{property_names:#?}");
        let property_idx = (property_names
            .iter()
            .position(|p| p.to_lowercase() == KVAR)
            .unwrap()
            + 1) as i32;
        let kw = dss_c::Obj_GetFloat64(element_ptr, property_idx.try_into().unwrap());
        Ok(kw)
    }
}

pub fn set_pf(value: f64) -> Result<()> {
    unsafe {
        let element_ptr = get_storage_ptr()?;
        let property_idx = (ckt_element::get_all_property_names()
            .iter()
            .position(|p| p.to_lowercase() == PF)
            .unwrap()
            + 1) as i32;
        dss_c::Obj_SetFloat64(element_ptr, property_idx, value);
        Ok(())
    }
}

pub fn get_pf() -> Result<f64> {
    unsafe {
        let element_ptr = get_storage_ptr()?;
        let property_names = ckt_element::get_all_property_names();
        println!("{property_names:#?}");
        let property_idx = (property_names
            .iter()
            .position(|p| p.to_lowercase() == PF)
            .unwrap()
            + 1) as i32;
        let kw = dss_c::Obj_GetFloat64(element_ptr, property_idx.try_into().unwrap());
        Ok(kw)
    }
}

fn get_storage_ptr() -> Result<*mut c_void> {
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

        Ok(element_ptr)
    }
}
