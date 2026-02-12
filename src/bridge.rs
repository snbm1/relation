#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_void, free as os_free};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/relation_bindings.rs"));

/// Забирает `char*` из Go, конвертирует в `String` и освобождает память.
/// ❗ Корректно, если Go возвращает строки через `C.CString`.
unsafe fn take_go_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    os_free(ptr as *mut c_void);
    Some(s)
}

/// &str → CString → *mut c_char (cgo ожидает `char*`)
fn to_c_mut(s: &str) -> CString {
    CString::new(s).expect("string contains NUL byte")
}

/* ---------------------------------------------------------
   SAFE API (ТО, ЧТО ТЫ БУДЕШЬ ИСПОЛЬЗОВАТЬ)
---------------------------------------------------------- */

pub fn setup_safe(
    basic_path: &str,
    working_path: &str,
    temp_path: &str,
    status_port: i64,
    debug: bool,
    enable_services: bool,
) -> Option<String> {
    let basic = to_c_mut(basic_path);
    let working = to_c_mut(working_path);
    let temp = to_c_mut(temp_path);

    unsafe {
        take_go_string(setup(
            basic.as_ptr() as *mut c_char,
            working.as_ptr() as *mut c_char,
            temp.as_ptr() as *mut c_char,
            status_port,
            debug as GoUint8,
            enable_services as GoUint8,
        ))
    }
}

pub fn parse_safe(content: &str, temp_path: &str) -> Option<String> {
    let c_content = to_c_mut(content);
    let c_temp = to_c_mut(temp_path);

    unsafe { take_go_string(parse(c_content.as_ptr() as *mut c_char, c_temp.as_ptr() as *mut c_char)) }
}

pub fn start_safe(config_path: &str, memory_limit: u8) -> Option<String> {
    let c_cfg = to_c_mut(config_path);

    unsafe { take_go_string(start(c_cfg.as_ptr() as *mut c_char, memory_limit as GoUint8)) }
}

pub fn restart_safe(config_path: &str, memory_limit: u8) -> Option<String> {
    let c_cfg = to_c_mut(config_path);

    unsafe { take_go_string(restart(c_cfg.as_ptr() as *mut c_char, memory_limit as GoUint8)) }
}

pub fn stop_safe() -> Option<String> {
    unsafe { take_go_string(stop()) }
}

pub fn url_test_safe(tag: &str) -> Option<String> {
    let c_tag = to_c_mut(tag);

    unsafe { take_go_string(urlTest(c_tag.as_ptr() as *mut c_char)) }
}

pub fn start_core_grpc_server_safe(listen_address: &str) -> Option<String> {
    let c_addr = to_c_mut(listen_address);

    unsafe { take_go_string(startCoreGrpcServer(c_addr.as_ptr() as *mut c_char)) }
}

pub fn enable_system_proxy_safe(
    host: &str,
    port: i64,
    support_socks: bool,
) -> Option<String> {
    let host_c = to_c_mut(host);

    unsafe {
        take_go_string(enableSystemProxy(
            host_c.as_ptr() as *mut c_char,
            port,
            support_socks as GoUint8,
        ))
    }
}

pub fn disable_system_proxy_safe() -> Option<String> {
    unsafe {
        take_go_string(disableSystemProxy())
    }
}
