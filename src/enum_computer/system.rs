pub mod system_enum {

    use winreg;
    use whoami;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
    use sysinfo::{System, SystemExt};

    // Enumerate system information
    pub fn system_information() 
    {
    // Get the Kernel/OS Version from the subkey
        let sys = System::new_all();
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_hklm = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ)
                            .expect("Error ! Failed to open the subkey SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
        let product_name: String = subkey_hklm.get_value("ProductName")
                            .expect("Error ! Failed to determine version, could not open the subkey ProductName");
        println!("\n • SYSTEM INFORMATION \n");
        println!("      OS: {}", product_name);
        println!("      Kernel version: {:?}", sys.kernel_version());
        println!("      Username: {}", whoami::username());
        println!("      Hostname: {}", whoami::hostname());
        println!("      Language: {:?} \n", whoami::lang().collect::<Vec<String>>());
    }
}