use crate::as_string;
use crate::duckly::{
    duckdb_function_get_bind_data, duckdb_function_get_init_data, duckdb_function_info,
    duckdb_function_set_error,
};
use std::os::raw::c_char;

#[derive(Debug)]
pub struct FunctionInfo(duckdb_function_info);

impl FunctionInfo {
    /// Report that an error has occurred while executing the function.
    ///
    /// # Arguments
    ///  * `error`: The error message
    pub fn set_error(&self, error: &str) {
        unsafe {
            duckdb_function_set_error(self.0, as_string!(error));
        }
    }
    pub fn get_bind_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_bind_data(self.0) as *mut T }
    }
    pub fn get_init_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_init_data(self.0) as *mut T }
    }
}

impl From<duckdb_function_info> for FunctionInfo {
    fn from(ptr: duckdb_function_info) -> Self {
        Self(ptr)
    }
}
