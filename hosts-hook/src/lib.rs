use std::borrow::Cow;
use std::{env, ptr};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::IpAddr;
use std::path::{Component, Path, PathBuf};
use std::sync::Once;
use libc::{addrinfo, c_char, c_int, hostent, in6_addr, in_addr, sa_family_t, sockaddr, sockaddr_in, sockaddr_in6, AF_INET, AF_INET6, SOCK_STREAM};
use log::debug;
use simple_logger::SimpleLogger;

mod platform;

#[cfg(target_os = "macos")]
pub static BUILD_LIB_NAME: &str = concat!("lib", env!("CARGO_CRATE_NAME"), ".dylib");

#[cfg(target_os = "linux")]
pub static BUILD_LIB_NAME: &'static str = concat!("lib", env!("CARGO_CRATE_NAME"), ".so");

pub static LOGGER_INIT: Once = Once::new();

#[derive(Debug, Clone, Copy)]
pub enum OsType {
    MacOS,
    Linux,
    Windows
}

/// <https://doc.rust-lang.org/std/env/consts/constant.OS.html>
pub fn get_os() -> OsType {
    match env::consts::OS {
        "macos" => OsType::MacOS,
        "linux" | "freebsd" => OsType::Linux,
        "windows" => OsType::Windows,
        _ => panic!("Unsupported OS")
    }
}

/// Normalizes a path without existing symlinks.
/// 
/// <https://github.com/rust-lang/cargo/blob/fede83ccf973457de319ba6fa0e36ead454d2e20/src/cargo/util/paths.rs#L61C1-L86C2>
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

struct HostsUpwardFinder<'a> {
    path: Option<&'a Path>,
}

impl<'a> HostsUpwardFinder<'a> {
    pub fn new(curr_dir: &'a Path) -> Self {
        Self {
            path: Some(curr_dir),
        }
    }

    pub fn find(&mut self, hostname: &str, env: Option<&str>) -> Option<IpAddr> {
        let filenames = [
            env.map(|str| Cow::Owned(format!("hosts.{str}"))),
            env.map(|str| Cow::Owned(format!(".hosts.{str}"))),
            Some(Cow::Borrowed("hosts")),
            Some(Cow::Borrowed(".hosts"))
        ].into_iter()
            .flatten()
            .collect::<Vec<Cow<str>>>();

        for path in self {
            for filename in &filenames {
                if filename.is_empty() {
                    continue;
                }

                let filepath = path.join(filename.as_ref());
                let Ok(file) = File::open(&filepath) else {
                    debug!("Not found: {}", filepath.to_str().unwrap());
                    continue;
                };

                debug!("Found: {}", filepath.to_str().unwrap());
                let reader = BufReader::new(file);

                if let Some(ipaddr) = find(reader, hostname) {
                    debug!("Found IpAddr: {ipaddr}");
                    return Some(ipaddr)
                }
            }
        }

        None
    }
}

impl<'a> Iterator for HostsUpwardFinder<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(path) = self.path else {
            return None;
        };

        self.path = path.parent();
        Some(path)
    }
}

fn find_by_hostname(hostname: &str, env: Option<&str>) -> Option<IpAddr> {
    LOGGER_INIT.call_once(|| SimpleLogger::new().init().unwrap());

    let curr_dir = std::env::current_dir().unwrap_or_default();
    let mut finder = HostsUpwardFinder::new(&curr_dir);
    finder.find(hostname, env)
}

fn find<R>(reader: BufReader<R>, hostname: &str) -> Option<IpAddr> 
where R: Read
{
    for line in reader.lines() {
        let Ok(str) = line else {
            return None;
        };

        if str.is_empty() || str.starts_with('#') {
            continue;
        }

        let Some((ip, hosts)) = str.split_once(' ') else {
            continue;
        };

        let Ok(ip_addr) = ip.parse::<IpAddr>() else {
            continue;
        };

        if let Some(host) = hosts.split_whitespace().next() {
            if host == hostname {
                return Some(ip_addr);
            }

            return Some(ip_addr);
        }
    }

    None
}

unsafe fn hook_gethostbyname(name: *const c_char) -> Option<*mut hostent> {
    let env = std::env::var("HOSTS_ENV").ok();
    let hostname = CStr::from_ptr(name).to_string_lossy();

    if let Some(ipaddr) = find_by_hostname(&hostname, env.as_deref()) {
        debug!("Hooked gethostbyname for: {hostname} -> {ipaddr}");
        return Some(ipaddr_to_hostent(&hostname, ipaddr));
    }
    debug!("No IP address found for gethostbyname: {hostname}");

    None
}

unsafe fn hook_gethostbyname2(name: *const c_char, af: c_int) -> Option<*mut hostent> {
    let env = std::env::var("HOSTS_ENV").ok();
    let hostname = CStr::from_ptr(name).to_string_lossy();

    if let Some(ipaddr) = find_by_hostname(&hostname, env.as_deref()) {
        debug!("Hooked gethostbyname2 for: {hostname} -> {ipaddr}");
        return Some(ipaddr_to_hostent(&hostname, ipaddr));
    }
    debug!("No IP address found for gethostbyname2: {hostname}");

    None
}

unsafe fn hook_getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> Option<()> {
    if node.is_null() {
        return None;
    }

    let env = std::env::var("HOSTS_ENV").ok();
    let hostname = CStr::from_ptr(node).to_string_lossy();

    let Some(ipaddr) = find_by_hostname(&hostname, env.as_deref()) else {
        debug!("No IP address found for getaddrinfo: {hostname}");
        return None;
    };

    let addrinfo = ipaddr_to_addrinfo(&hostname, ipaddr);
    if addrinfo.is_null() {
        debug!("Failed to convert IP address to addrinfo for: {hostname}");
        return None;
    }

    debug!("Hooked getaddrinfo for: {hostname} -> {ipaddr}");

    *res = addrinfo;
    Some(())
}

unsafe fn ipaddr_to_hostent(hostname: &str, ipaddr: IpAddr) -> *mut hostent {
    let host = libc::malloc(size_of::<hostent>()).cast::<hostent>();
    if host.is_null() {
        return ptr::null_mut();
    }

    let cname = CString::new(hostname.to_string()).unwrap();
    (*host).h_name = libc::strdup(cname.as_ptr());

    if let IpAddr::V4(ipv4) = ipaddr {
        (*host).h_addrtype = AF_INET;
        (*host).h_length = size_of::<in_addr>() as i32;

        let inaddr = libc::malloc(size_of::<in_addr>()).cast::<in_addr>();
        if inaddr.is_null() {
            return ptr::null_mut();
        }
        (*inaddr).s_addr = u32::from_ne_bytes(ipv4.octets());
        
        let addr_list = libc::malloc(2 * size_of::<*mut c_char>()).cast::<*mut c_char>();
        *addr_list = inaddr.cast::<c_char>();
        *addr_list.add(1) = ptr::null_mut();
        (*host).h_addr_list = addr_list;
    } else if let IpAddr::V6(ipv6) = ipaddr {
        (*host).h_addrtype = AF_INET6;
        (*host).h_length = size_of::<in6_addr>() as i32;

        let in6addr = libc::malloc(size_of::<in6_addr>()).cast::<in6_addr>();
        if in6addr.is_null() {
            return ptr::null_mut();
        }
        (*in6addr).s6_addr = ipv6.octets();

        let addr_list = libc::malloc(2 * size_of::<*mut c_char>()).cast::<*mut c_char>();
        *addr_list = in6addr.cast::<c_char>();
        *addr_list.add(1) = ptr::null_mut();
        (*host).h_addr_list = addr_list;
    }

    let alias_list = libc::malloc(size_of::<*mut c_char>()).cast::<*mut c_char>();
    *alias_list = ptr::null_mut();
    (*host).h_aliases = alias_list;

    host
}

unsafe fn ipaddr_to_addrinfo(hostname: &str, ipaddr: IpAddr) -> *mut addrinfo {
    let ai = libc::malloc(size_of::<addrinfo>()).cast::<addrinfo>();
    if ai.is_null() {
        return ptr::null_mut();
    }

    (*ai).ai_flags = 0;
    (*ai).ai_family = match ipaddr {
        IpAddr::V4(_) => AF_INET,
        IpAddr::V6(_) => AF_INET6,
    };
    (*ai).ai_socktype = SOCK_STREAM;
    (*ai).ai_protocol = 0;
    (*ai).ai_addrlen = match ipaddr {
        IpAddr::V4(_) => size_of::<sockaddr_in>() as u32,
        IpAddr::V6(_) => size_of::<sockaddr_in6>() as u32,
    };
    (*ai).ai_canonname = libc::strdup(CString::new(hostname).unwrap().as_ptr());
    (*ai).ai_next = ptr::null_mut();

    match ipaddr {
        IpAddr::V4(ipv4) => {
            let sockaddr_ptr = libc::malloc(size_of::<sockaddr_in>()).cast::<sockaddr_in>();
            if sockaddr_ptr.is_null() {
                return ptr::null_mut();
            }

            (*sockaddr_ptr).sin_family = AF_INET as sa_family_t;
            (*sockaddr_ptr).sin_port = 0;
            (*sockaddr_ptr).sin_addr.s_addr = u32::from_ne_bytes(ipv4.octets());

            (*ai).ai_addr = sockaddr_ptr.cast::<sockaddr>();
        }
        IpAddr::V6(ipv6) => {
            let sockaddr6_ptr = libc::malloc(size_of::<sockaddr_in6>()).cast::<sockaddr_in6>();
            if sockaddr6_ptr.is_null() {
                return ptr::null_mut();
            }

            (*sockaddr6_ptr).sin6_family = AF_INET6 as sa_family_t;
            (*sockaddr6_ptr).sin6_port = 0;
            (*sockaddr6_ptr).sin6_flowinfo = 0;
            (*sockaddr6_ptr).sin6_scope_id = 0;
            (*sockaddr6_ptr).sin6_addr.s6_addr = ipv6.octets();

            (*ai).ai_addr = sockaddr6_ptr.cast::<sockaddr>();
        }
    }

    ai
}