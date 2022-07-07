pub mod network_enum {

    use ipconfig;
    use netstat2::*;

    pub fn network_information() 
    {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags)
        .expect("Error ! Could not acccess socket infos ! ");
        println!("\n • NETWORK \n");
        println!("\n    IP addresses  \n");
    // Get all the adapters and return their IP addresses    
        for adapter in ipconfig::get_adapters()
                                .expect("Error ! Could not open the adaptator")
        {
            println!("      {:} - {:?}", adapter.description(), adapter.ip_addresses());
        }

        println!("\n    DNS server  \n");
    // DNS Server
        for adapter in ipconfig::get_adapters()
                                .expect("Could not open the adaptator")
        {
            println!("      {:?}", adapter.dns_servers());
        }
    // Return Ports in use
        println!("\n    NETWORK STATISTICS   \n");
        for si in sockets_info {
            match si.protocol_socket_info 
            {
                ProtocolSocketInfo::Tcp(tcp_si) => println!(
                    "      TCP {}:{} -> {}:{} {:?} - {}",
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                    si.associated_pids,
                    tcp_si.state
                ),
                ProtocolSocketInfo::Udp(udp_si) => println!(
                    "      UDP {}:{} -> *:* {:?}",
                    udp_si.local_addr, udp_si.local_port, si.associated_pids
                ),
            }
        }
    }
}