//! Utility create to enumerate local network interfaces.
//!
//! This crate was tested on Windows 10 and Ubuntu 19.10
//!
//! # Example
//!
//! ```
//! use netifs::get_interfaces;
//!
//! fn main() {
//!     for interface in get_interfaces().expect("Getting interfaces failed") {
//!         println!("{}", interface.name);
//!         if let Some(mac) = interface.mac_address {
//!             println!("\tMAC: {}", mac.to_hex_string());
//!         }
//!         for ip in interface.ip_addresses {
//!             println!("\tIP: {}", ip);
//!         }
//!     }
//! }

use std::ffi::CStr;
use libc::c_char;
use eui48::MacAddress;
use ipnetwork::IpNetwork;

#[cfg(windows)]
mod winapi_um_iptypes;
#[cfg(windows)]
mod winapi_shared_ifdef;

/// Represents a network interface.
#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub display_name: String,
    pub ip_addresses: Vec<IpNetwork>,
    pub mac_address: Option<MacAddress>
}

impl Interface {
    /// Create a new interface with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            display_name: name,
            ip_addresses: Vec::new(),
            mac_address: None
        }
    }
}

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

#[cfg(not(windows))]
#[path = "unix.rs"]
mod platform;

pub(crate) fn cstr_to_string(cstr: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(cstr).to_string_lossy().to_owned().to_string()
    }
}

/// Retrieve the network interfaces.
pub fn get_interfaces() -> Result<Vec<Interface>, String> {
    platform::get_interfaces()
}