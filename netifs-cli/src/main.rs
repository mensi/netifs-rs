use netifs::get_interfaces;

fn main() {
    for interface in get_interfaces().expect("Getting interfaces failed") {
        println!("{}", interface.display_name);
        if interface.is_loopback {
            println!("\t[loopback interface]");
        }
        if !interface.is_up {
            print!("\t[interface is not up!]");
        }
        if let Some(mac) = interface.mac_address {
            println!("\tMAC: {}", mac.to_hex_string());
        }
        for ip in interface.ip_addresses {
            println!("\tIP: {}", ip);
        }
    }
}
