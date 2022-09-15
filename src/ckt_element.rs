extern crate dss_rs_sys;
use crate::dss_result::{DssError, Result};
use dss_rs_sys as dss_c;
use std::{ptr, slice};

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
