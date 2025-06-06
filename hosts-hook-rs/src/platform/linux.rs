mod linux {
    use libc::{addrinfo, c_char, c_int, hostent, RTLD_NEXT};
    use std::ffi::CStr;
    use std::ptr;
    use std::sync::Once;
    use crate::{hook_getaddrinfo, hook_gethostbyname, hook_gethostbyname2};

    static INIT: Once = Once::new();
    static mut GETHOSTBYNAME_FN: Option<unsafe extern "C" fn(*const c_char) -> *mut hostent> = None;
    static mut GETHOSTBYNAME2_FN: Option<unsafe extern "C" fn(*const c_char, c_int) -> *mut hostent> = None;
    static mut GETADDRINFO_FN: Option<
        unsafe extern "C" fn(
            *const c_char,
            *const c_char,
            *const addrinfo,
            *mut *mut addrinfo,
        ) -> c_int,
    > = None;

    unsafe fn init() {
        GETHOSTBYNAME_FN = Some(std::mem::transmute(libc::dlsym(
            RTLD_NEXT,
            b"gethostbyname\0".as_ptr() as *const _,
        )));
        GETHOSTBYNAME2_FN = Some(std::mem::transmute(libc::dlsym(
            RTLD_NEXT,
            b"gethostbyname2\0".as_ptr() as *const _,
        )));
        GETADDRINFO_FN = Some(std::mem::transmute(libc::dlsym(
            RTLD_NEXT,
            b"getaddrinfo\0".as_ptr() as *const _,
        )));
    }

    #[no_mangle]
    pub unsafe extern "C" fn gethostbyname(name: *const c_char) -> *mut hostent {
        INIT.call_once(|| init());

        if let Some(hostenv) = hook_gethostbyname(name) {
            return hostenv;
        }

        match GETHOSTBYNAME_FN {
            Some(func) => func(name),
            None => ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn gethostbyname2(name: *const c_char, af: c_int) -> *mut hostent {
        INIT.call_once(|| init());

        if let Some(hostent) = hook_gethostbyname2(name, af) {
            return hostent;
        }

        match GETHOSTBYNAME_FN {
            Some(func) => func(name),
            None => ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int {
        INIT.call_once(|| init());

        if let Some(_) = hook_getaddrinfo(node, service, hints, res) {
            return 0;
        }

        match GETADDRINFO_FN {
            Some(func) => func(node, service, hints, res),
            None => -1,
        }
    }
}