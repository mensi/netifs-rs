# netifs

This is a simple rust wrapper on top of libc/winapi to enumerate
network interfaces. On Windows, an MSVC toolchain is necessary
to build.

Loopback interface and IPv6 support is present in both platform
specific implementations.

## Usage

An small example program is provided in netifs-cli:

```
use netifs::get_interfaces;

fn main() {
    for interface in get_interfaces().expect("Getting interfaces failed") {
        println!("{}", interface.display_name);
        if let Some(mac) = interface.mac_address {
            println!("\tMAC: {}", mac.to_hex_string());
        }
        for ip in interface.ip_addresses {
            println!("\tIP: {}", ip);
        }
    }
}
```