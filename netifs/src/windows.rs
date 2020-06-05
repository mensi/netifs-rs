use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use eui48::MacAddress;
use ipnetwork::IpNetwork;

use winapi::shared::winerror::*;
use winapi::shared::ntdef::{ULONG, PVOID};
use winapi::shared::ws2def::{AF_INET, AF_INET6, SOCKADDR_IN};
use winapi::shared::ws2ipdef::SOCKADDR_IN6_LH;
use crate::winapi_um_iptypes::*; // TODO replace with upstream winapi once released.

use widestring::U16CString;

use crate::{Interface, cstr_to_string};

#[link(name = "IPHLPAPI")]
extern {
    fn GetAdaptersAddresses(family: ULONG, flags: ULONG, reserved: PVOID, buffer: *const u8, size: *mut ULONG) -> ULONG;
}

pub fn get_interfaces() -> Result<Vec<Interface>, String> {
    unsafe {
        let mut size: ULONG = 1024;
        let mut buffer = Vec::with_capacity(size as usize);

        let sizeptr: *mut ULONG = &mut size;
        let mut res = GetAdaptersAddresses(0, 0, 0 as PVOID, buffer.as_mut_ptr(), sizeptr);

        // Since we are providing the buffer, it might be too small. Check for overflow
        // and try again with the required buffer size. There is a chance for a race 
        // condition here if an interface is added between the two calls - however
        // looping potentially forever seems more dangerous.
        if res == ERROR_BUFFER_OVERFLOW {
            buffer.reserve(size as usize - buffer.len());
            res = GetAdaptersAddresses(0, 0, 0 as PVOID, buffer.as_mut_ptr(), sizeptr);
        }

        if res != ERROR_SUCCESS {
            return Err(format!("GetAdaptersAddresses failed: {}", res));
        }

        let mut res = Vec::new();
        let mut adapterptr = buffer.as_ptr() as PIP_ADAPTER_ADDRESSES ;
        while !adapterptr.is_null() {
            let a = *adapterptr;
            
            let name = cstr_to_string(a.AdapterName);
            let mut interface = Interface::new(name);
            interface.display_name = U16CString::from_ptr_str(a.Description).to_string_lossy();

            if a.PhysicalAddressLength == 0 {
                // 0 indicates no MAC address, eg. on the loopback
            } else if a.PhysicalAddressLength != 6 {
                eprintln!("Interface '{}' has an unexpected address length: {}",
                          &interface.name, a.PhysicalAddressLength);
            } else {
                interface.mac_address = match MacAddress::from_bytes(&a.PhysicalAddress[..6]) {
                    Ok(m) => Some(m),
                    Err(()) => {
                        eprintln!("Unable to parse mac address for interface: {}", &interface.name);
                        None
                    }
                };
            }

            let mut current = a.FirstUnicastAddress;
            while !current.is_null() {
                let addr = *current;
                match (*addr.Address.lpSockaddr).sa_family as i32 {
                    AF_INET => {
                        let sin = *(addr.Address.lpSockaddr as *const SOCKADDR_IN);
                        let ip = IpAddr::V4(Ipv4Addr::from(u32::from_be(*sin.sin_addr.S_un.S_addr())));
                        match IpNetwork::new(ip, addr.OnLinkPrefixLength) {
                            Ok(ipn) => interface.ip_addresses.push(ipn),
                            Err(e) => return Err(format!("Unable to assemble IpNetwork: {}", e))
                        }
                    },
                    AF_INET6 => {
                        let sin6 = *(addr.Address.lpSockaddr as *const SOCKADDR_IN6_LH);
                        let ip = IpAddr::V6(Ipv6Addr::from(*sin6.sin6_addr.u.Byte()));
                        match IpNetwork::new(ip, addr.OnLinkPrefixLength) {
                            Ok(ipn) => interface.ip_addresses.push(ipn),
                            Err(e) => return Err(format!("Unable to assemble IpNetwork: {}", e))
                        }
                    },
                    x => {
                        eprintln!("Unknown sa_family: {}", x);
                    }
                }

                assert_ne!(current, addr.Next);
                current = addr.Next;
            }          

            res.push(interface);
            assert_ne!(adapterptr, a.Next);
            adapterptr = a.Next;
        }

        Ok(res)
    }
}
