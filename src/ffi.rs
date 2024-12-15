
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use crate::{rpc_call, api_call, run_substream};

// Create a global tokio runtime
lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[no_mangle]
pub extern "C" fn rpc_call_ffi(
    rpc_endpoint: *const c_char,
    method: *const c_char,
    params_input: *const c_char,
    id: i32,
) -> *mut c_char {
    if rpc_endpoint.is_null() || method.is_null() || params_input.is_null() {
        return CString::new("Null pointer passed").unwrap().into_raw();
    }

    let rpc_endpoint = unsafe { CStr::from_ptr(rpc_endpoint).to_string_lossy().to_string() };
    let method = unsafe { CStr::from_ptr(method).to_string_lossy().to_string() };
    let params_input = unsafe { CStr::from_ptr(params_input).to_string_lossy().to_string() };

    let result = RUNTIME.block_on(rpc_call(&rpc_endpoint, &method, &params_input, id));

    match result {
        Ok(value) => CString::new(value.to_string()).unwrap().into_raw(),
        Err(err) => CString::new(format!("Error: {}", err)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn api_call_ffi(
    api_url: *const c_char,
    optional_headers: *const c_char,
) -> *mut c_char {
    if api_url.is_null() {
        return CString::new("Null pointer passed").unwrap().into_raw();
    }

    let api_url = unsafe { CStr::from_ptr(api_url).to_string_lossy().to_string() };
    let optional_headers = unsafe {
        if optional_headers.is_null() {
            None
        } else {
            Some(CStr::from_ptr(optional_headers).to_string_lossy().to_string())
        }
    };

    let result = RUNTIME.block_on(api_call(&api_url, optional_headers.as_deref()));

    match result {
        Ok(response) => CString::new(response).unwrap().into_raw(),
        Err(err) => CString::new(format!("Error: {}", err)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn run_substream_ffi(
    endpoint_url: *const c_char,
    package_file: *const c_char,
    module_name: *const c_char,
    range: *const c_char,
) -> *mut c_char {
    if endpoint_url.is_null() || package_file.is_null() || module_name.is_null() {
        return CString::new("Null pointer passed").unwrap().into_raw();
    }

    let endpoint_url = unsafe { CStr::from_ptr(endpoint_url).to_string_lossy().to_string() };
    let package_file = unsafe { CStr::from_ptr(package_file).to_string_lossy().to_string() };
    let module_name = unsafe { CStr::from_ptr(module_name).to_string_lossy().to_string() };
    let range = unsafe {
        if range.is_null() {
            None
        } else {
            Some(CStr::from_ptr(range).to_string_lossy().to_string())
        }
    };

    let result = RUNTIME.block_on(run_substream(endpoint_url, &package_file, &module_name, range));

    match result {
        Ok(debug_values) => {
            let output = format!("{:?}", debug_values);
            CString::new(output).unwrap().into_raw()
        }
        Err(err) => CString::new(format!("Error: {}", err)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        println!("free_string: Received null pointer, skipping free.");
        return;
    }
    unsafe {
        println!("free_string: Freeing pointer {:?}", ptr);
        CString::from_raw(ptr); // Safely deallocates the memory
    }
}
