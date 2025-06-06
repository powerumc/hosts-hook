mod macos {
    use crate::{hook_getaddrinfo, hook_gethostbyname, hook_gethostbyname2};
    use libc::{addrinfo, c_char, c_int, hostent};

    extern "C" {
        fn gethostbyname(name: *const c_char) -> *mut hostent;
    }
    extern "C" {
        fn gethostbyname2(name: *const c_char, af: c_int) -> *mut hostent;
    }
    extern "C" {
        fn getaddrinfo(node: *const c_char,
                       service: *const c_char,
                       hints: *const addrinfo,
                       res: *mut *mut addrinfo,
        ) -> c_int;
    }

    #[repr(C)]
    struct Interpose<T> {
        replacement: T,
        original: T,
    }

    #[used]
    #[link_section = "__DATA,__interpose"]
    static INTERPOSE_GETHOSTBYNAME: Interpose<unsafe extern "C" fn(*const c_char) -> *mut hostent> =
        Interpose {
            replacement: my_gethostbyname,
            original: gethostbyname,
        };

    #[used]
    #[link_section = "__DATA,__interpose"]
    static INTERPOSE_GETADDRINFO: Interpose<
        unsafe extern "C" fn(
            *const c_char,
            *const c_char,
            *const addrinfo,
            *mut *mut addrinfo,
        ) -> c_int,
    > = Interpose {
        replacement: my_getaddrinfo,
        original: getaddrinfo,
    };
    
    #[used]
    #[link_section = "__DATA,__interpose"]
    static INTERPOSE_GETHOSTBYNAME2: Interpose<unsafe extern "C" fn(*const c_char, c_int) -> *mut hostent> =
        Interpose {
            replacement: my_gethostbyname2,
            original: gethostbyname2,
        };

    #[no_mangle]
    pub unsafe extern "C" fn my_gethostbyname(name: *const c_char) -> *mut hostent {
        if let Some(hostent) = hook_gethostbyname(name) {
            return hostent;
        }
        
        gethostbyname(name)
    }
    
    #[no_mangle]
    pub unsafe extern "C" fn my_gethostbyname2(name: *const c_char, af: c_int) -> *mut hostent {
        if let Some(hostent) = hook_gethostbyname2(name, af) {
            return hostent;
        }
        
        gethostbyname2(name, af)
    }

    #[no_mangle]
    pub unsafe extern "C" fn my_getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int {
        if hook_getaddrinfo(node, service, hints, res).is_some() {
            return 0;
        }

        getaddrinfo(node, service, hints, res)
    }

}