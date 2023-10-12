extern crate dss_rs_sys;
use crate::dss_result::{DssError, Result};
use dss_rs_sys as dss_c;
use std::{ptr, slice};

pub fn enable() {
    unsafe {
        dss_c::CktElement_Set_Enabled(true.into());
    }
}

pub fn disable() {
    unsafe {
        dss_c::CktElement_Set_Enabled(false.into());
    }
}

pub fn open(term: i32, phs: i32) {
    unsafe {
        dss_c::CktElement_Open(term, phs);
    }
}

pub fn close(term: i32, phs: i32) {
    unsafe {
        dss_c::CktElement_Close(term, phs);
    }
}

pub fn get_powers() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_Powers(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_voltages_mag_ang() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_VoltagesMagAng(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_currents_mag_ang() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_CurrentsMagAng(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn get_total_powers() -> Result<Vec<f64>> {
    unsafe {
        let mut result_cnt = 0;
        let mut result_ptr = ptr::null_mut();
        dss_c::CktElement_Get_TotalPowers(&mut result_ptr, &mut result_cnt);
        if result_cnt == 0 || result_ptr == ptr::null_mut() {
            return Err(DssError::NullCPtr);
        }
        let v = slice::from_raw_parts(result_ptr, result_cnt as usize).to_vec();
        Ok(v)
    }
}

pub fn is_open(term: i32, phs: i32) -> u16 {
    unsafe {
        dss_c::CktElement_IsOpen(term, phs)
    }
}
