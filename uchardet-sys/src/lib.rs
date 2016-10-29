//! Low-level, unsafe wrapper for the uchardet API.  Not intended to be
//! used directly, unless you're writing your own wrapper.

extern crate libc;

use libc::{c_char, c_int, c_void, size_t};

#[allow(non_camel_case_types)]
pub type uchardet_t = *mut c_void;

#[allow(non_camel_case_types)]
pub type nsresult = c_int;

extern {
    pub fn uchardet_new() -> uchardet_t;
    pub fn uchardet_delete(ud: uchardet_t);
    pub fn uchardet_handle_data(ud: uchardet_t, data: *const c_char,
                                len: size_t) -> nsresult;
    pub fn uchardet_data_end(ud: uchardet_t);
    pub fn uchardet_reset(ud: uchardet_t);
    pub fn uchardet_get_charset(ud: uchardet_t) -> *const c_char;
}
