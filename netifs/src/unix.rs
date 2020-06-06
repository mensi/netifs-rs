use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use libc;
use eui48::MacAddress;
use ipnetwork::IpNetwork;

use crate::{Interface, cstr_to_string};

pub fn get_interfaces() -> Result<Vec<Interface>, String> {
    let mut res: HashMap<String, Interface> = HashMap::new();

    unsafe {
        let mut addrptr: *mut libc::ifaddrs = 0 as *mut libc::ifaddrs;
        let ret = libc::getifaddrs(&mut addrptr);
        if ret != 0 {
            return Err(format!("getifaddrs call failed with code: {}", ret));
        }

        let mut current = addrptr;
        while !current.is_null() {
            let addr = *current;
            let name = cstr_to_string(addr.ifa_name);

            let interface = res.entry(name.clone()).or_insert(Interface::new(name.clone()));
            if addr.ifa_flags & libc::IFF_LOOPBACK as u32 != 0 {
                interface.is_loopback = true;
            }
            if addr.ifa_flags & libc::IFF_UP as u32 != 0 {
                interface.is_up = true;
            }

            match (*addr.ifa_addr).sa_family as libc::c_int {
                libc::AF_PACKET => {
                    let sll = *(addr.ifa_addr as *const libc::sockaddr_ll);
                    interface.mac_address = match MacAddress::from_bytes(&sll.sll_addr[..6]) {
                        Ok(m) => Some(m),
                        Err(_) => {eprintln!("Unable to parse mac address on {}", name); None}
                    };
                },
                libc::AF_INET => {
                    let sin = *(addr.ifa_addr as *const libc::sockaddr_in);
                    let sin_mask = *(addr.ifa_netmask as *const libc::sockaddr_in);
                    assert_eq!(sin.sin_family, sin_mask.sin_family);
                    let ip = IpAddr::V4(Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr)));
                    let mask = IpAddr::V4(Ipv4Addr::from(u32::from_be(sin_mask.sin_addr.s_addr)));

                    match IpNetwork::with_netmask(ip, mask) {
                        Ok(ipn) => interface.ip_addresses.push(ipn),
                        Err(e) => return Err(format!("Unable to construct IpNetwork: {}", e))
                    }
                },
                libc::AF_INET6 => {
                    let sin6 = *(addr.ifa_addr as *const libc::sockaddr_in6);
                    let sin6_mask = *(addr.ifa_netmask as *const libc::sockaddr_in6);
                    assert_eq!(sin6.sin6_family, sin6_mask.sin6_family);
                    let ip = IpAddr::V6(Ipv6Addr::from(sin6.sin6_addr.s6_addr));
                    let mask = IpAddr::V6(Ipv6Addr::from(sin6_mask.sin6_addr.s6_addr));
                    
                    match IpNetwork::with_netmask(ip, mask) {
                        Ok(ipn) => interface.ip_addresses.push(ipn),
                        Err(e) => return Err(format!("Unable to construct IpNetwork: {}", e))
                    }
                },
                x => {
                    eprintln!("Unknown sa_family: {}", x);
                }
            }

            assert_ne!(current, addr.ifa_next);
            current = addr.ifa_next;
        }

        libc::freeifaddrs(addrptr);
    }

    Ok(res.into_iter().map(|(_key, val)| val).collect())
}