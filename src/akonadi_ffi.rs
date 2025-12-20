use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[link(name = "akonadi_helper")]
extern "C" {
    fn modify_pimitem(item_id: i64, remote_id: *const c_char) -> i32;
    fn delete_pimitem(item_id: i64) -> i32;
    fn get_last_error() -> *const c_char;
    fn cleanup_qt_app();
}

pub fn modify_item(item_id: i64, remote_id: &str) -> Result<(), String> {
    let remote_id_c = CString::new(remote_id).map_err(|e| format!("Invalid remote ID: {}", e))?;

    let result = unsafe { modify_pimitem(item_id, remote_id_c.as_ptr()) };

    if result != 0 {
        let error = unsafe {
            let err_ptr = get_last_error();
            if err_ptr.is_null() {
                "Unknown error".to_string()
            } else {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            }
        };
        return Err(error);
    }

    Ok(())
}

pub fn delete_item(item_id: i64) -> Result<(), String> {
    let result = unsafe { delete_pimitem(item_id) };

    if result != 0 {
        let error = unsafe {
            let err_ptr = get_last_error();
            if err_ptr.is_null() {
                "Unknown error".to_string()
            } else {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            }
        };
        return Err(error);
    }

    Ok(())
}

pub fn cleanup() {
    unsafe { cleanup_qt_app() };
}
