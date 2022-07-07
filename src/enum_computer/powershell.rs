pub mod powershell_enum {

    use winreg;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

    pub fn powershell_version()
    {
        println!("\n • PowerShell version available \n");
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let v2 = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\PowerShell\1\PowerShellEngine"#, KEY_READ)
                            .expect("Error ! Failed to open the subkey SOFTWARE\\Microsoft\\PowerShell\\1\\PowerShellEngine");
        let v5 = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\PowerShell\3\PowerShellEngine"#, KEY_READ)
                            .expect("Error ! Failed to open the subkey SOFTWARE\\Microsoft\\PowerShell\\3\\PowerShellEngine");

        for (name, value) in v2.enum_values()
                                .map(|x| x.expect("Error ! Could not enumerate value for PowerShell v2!")) 
        {
                            if name.contains("PowerShellVersion")
                            {
                                println!("      PowerShell v2 : True (version : {:})", value);
                            }
        }

        for (name, value) in v5.enum_values()
                                .map(|x| x.expect("Error ! Could not enumerate value for PowerShell v2!"))
        {
                            if name.contains("PowerShellVersion")
                            {
                                println!("      PowerShell v5 : True (version : {:})", value);
                            }
        }
    }
}