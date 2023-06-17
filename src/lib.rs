/*
* Copyright (c) 2023 XXIV
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::ffi::{CStr,CString};
use cleasy::App;

#[repr(C)]
struct cleasy_app_t;

#[no_mangle]
unsafe extern "C" fn cleasy_app_new(name: *const c_char, version: *const c_char, author: *const c_char) -> *mut cleasy_app_t {
    let name_rs = match CStr::from_ptr(name).to_str() {
	Ok(v) => v,
	Err(_) => return ptr::null_mut()
    };
    let version_rs = match CStr::from_ptr(version).to_str() {
	Ok(v) => v,
	Err(_) => return ptr::null_mut()
    };
    let author_rs = match CStr::from_ptr(author).to_str() {
	Ok(v) => v,
	Err(_) => return ptr::null_mut()
    };
    let app = App::new(name_rs, version_rs, author_rs);
    Box::into_raw(Box::new(app)) as *mut cleasy_app_t
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_arg_was_used(app: *const cleasy_app_t, arg: *const c_char) -> c_int {
    let ref app_rs = *(app as *const App);
    let arg_rs = match CStr::from_ptr(arg).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };
    if app_rs.arg_was_used(arg_rs) {
	return 1
    }
    0
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_add_arg(app: *mut cleasy_app_t, name: *const c_char, help: *const c_char, data: *const c_char) -> c_int {
    let ref mut app_rs = *(app as *mut App);
    let name_rs = match CStr::from_ptr(name).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };
    let help_rs = match CStr::from_ptr(help).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };
    let data_rs = match CStr::from_ptr(data).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };
    app_rs.add_arg(name_rs, help_rs, data_rs);
    0
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_get_arg_data(app: *const cleasy_app_t, name: *const c_char, is_err: *mut c_int) -> *mut c_char {
    let ref app_rs = *(app as *const App);
    let name_rs = match CStr::from_ptr(name).to_str() {
	Ok(v) => v,
	Err(_) => {
	    *is_err = -1;
	    return ptr::null_mut()
	}
    };
    match app_rs.get_arg_data(name_rs) {
	Ok(v) => match CString::new(v) {
	    Ok(inv) => {
		*is_err = 0;
		inv.into_raw()
	    },
	    Err(_) => {
		*is_err = -1;
		ptr::null_mut()
	    }
	},
	Err(v) => match CString::new(v.details) {
	    Ok(inv) => {
		*is_err = 1;
		inv.into_raw()
	    },
	    Err(_) => {
		*is_err = -1;
		ptr::null_mut()
	    }
	}
    }
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_version_is(app: *const cleasy_app_t) -> c_int {
    let ref app_rs = *(app as *const App);
    if app_rs.version_is() {
	return 1
    }
    0
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_help_is(app: *const cleasy_app_t) -> c_int {
    let ref app_rs = *(app as *const App);
    if app_rs.help_is() {
	return 1
    }
    0
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_version_info(app: *const cleasy_app_t) -> *mut c_char {
    let ref app_rs = *(app as *const App);
    match CString::new(app_rs.version_info()) {
	Ok(v) => v.into_raw(),
	Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_help_info(app: *const cleasy_app_t) -> *mut c_char {
    let ref app_rs = *(app as *const App);
    match CString::new(app_rs.help_info()) {
	Ok(v) => v.into_raw(),
	Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn cleasy_app_destroy(app: *mut cleasy_app_t) {
    if !app.is_null() {
	let _ = Box::from_raw(app);
    }
}

#[no_mangle]
unsafe extern "C" fn cleasy_string_destroy(ptr: *mut c_char) {
    if !ptr.is_null() {
	let _ = CString::from_raw(ptr);
    }
}
