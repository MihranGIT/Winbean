pub mod enum_comp { 

    use ipconfig;
    use winreg;
    use whoami;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
    use sysinfo::{ProcessExt, System, SystemExt};
    use netstat2::*;
    use chrono::prelude::*;
    
    pub fn network_information() 
    {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
        println!("\n • NETWORK \n");
        println!("\n    IP addresses  \n");
        
        for adapter in ipconfig::get_adapters()
                                .expect("Could not open the adaptator")
        {
            println!("      {:} - {:?}", adapter.description(), adapter.ip_addresses());
        }

        println!("\n    DNS server  \n");
        
        for adapter in ipconfig::get_adapters()
                                .expect("Could not open the adaptator")
        {
            println!("      {:?}", adapter.dns_servers());
        }

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

    pub fn system_information() 
    {
        let sys = System::new_all();
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_hklm = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ)
                            .expect("Failed to open subkey");
        let product_name: String = subkey_hklm.get_value("ProductName")
                            .expect("Failed to determine version");
        println!("\n • SYSTEM INFORMATION \n");
        println!("      OS: {}", product_name);
        println!("      Kernel version: {:?}", sys.kernel_version());
        println!("      Username: {}", whoami::username());
        println!("      Hostname: {}", whoami::hostname());
        println!("      Language: {:?} \n", whoami::lang().collect::<Vec<String>>());
    }

    pub fn powershell_version()
    {
        println!("\n • PowerShell version available \n");
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let v2 = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\PowerShell\1\PowerShellEngine"#, KEY_READ)
                            .expect("Failed to open subkey");
        let v5 = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\PowerShell\3\PowerShellEngine"#, KEY_READ)
                            .expect("Failed to open subkey");

        for (name, value) in v2.enum_values()
                                .map(|x| x.unwrap()) 
        {
                            if name.contains("PowerShellVersion")
                            {
                                println!("      PowerShell v2 : True (version : {:})", value);
                            }
        }

        for (name, value) in v5.enum_values()
                                .map(|x| x.unwrap()) 
        {
                            if name.contains("PowerShellVersion")
                            {
                                println!("      PowerShell v5 : True (version : {:})", value);
                            }
        }
    }

    pub fn processes()
    {
        let sys = System::new_all();
        println!("\n • RUNNING PROCESSES \n");
        for (pid, process) in sys.processes() 
        {
            println!("      [{}] - {} - {} (Path : {})", pid, 
                                                        process.status(), 
                                                        process.name(), 
                                                        process.exe().display());
        }
    }

    pub fn get_time()
    {
        let local: DateTime<Local> = Local::now();
        println!("\n • LOCAL TIME \n");
        println!("      {}", local);
    }
}