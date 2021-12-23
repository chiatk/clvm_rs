use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

pub mod allocator;
pub mod chia_dialect;
mod core_ops;
pub mod cost;
mod dialect;
mod err_utils;
pub mod f_table;
mod gen;
mod int_to_bytes;
pub mod more_ops;
pub mod node;
mod number;
mod op_utils;
pub mod operator_handler;
#[cfg(not(any(test, target_family = "wasm")))]
mod py;
pub mod reduction;
pub mod run_program;
pub mod serialize;
mod sha2;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_ops;

#[cfg(target_family = "wasm")]
pub mod wasm;


#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

