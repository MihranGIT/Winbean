use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, HKEY_USERS, KEY_READ};
use whoami;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    basic_information();
    browse_dir();
}

fn basic_information() {
    println!("==================================================================");
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let _hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let _hku = winreg::RegKey::predef(HKEY_USERS);
    let subkey_hklm = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ).expect("Failed to open subkey");
    let product_name: String = subkey_hklm.get_value("ProductName").expect("Failed to determine version");
    println!("Windows version: {}", product_name);
    println!("Username: {}", whoami::username());
    println!("Hostname: {}", whoami::hostname());
    println!("==================================================================");
}

fn browse_dir() {
    for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
        let path = file.path().display();
        let file = file.file_name().to_string_lossy();
        if file.ends_with("password.txt")
        {
            println!("Fichier password trouve : {}", path);
            let file = File::open(path.to_string()).expect("Impossible d'ouvrir le fichier");
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                println!("Contenu : {}", line);
            }
        }
        if file.ends_with(".gitconfig")
        {
            println!("Fichier config git : {}", path);
            let file = File::open(path.to_string()).expect("toto");
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                println!("Contenu : {}", line);
            }
        }
        if file.ends_with(".bash_history")
        {
            println!("Fichier d'historique WSL trouve : {}", path);
            let file = File::open(path.to_string()).expect("toto");
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                println!("Contenu : {}", line);
            } 
        }
    }
}


// ####### VNC #################
// RealVNC : HKEY_LOCAL_MACHINE\SOFTWARE\RealVNC\vncserver, Value: Password
// TightVNC : HKEY_CURRENT_USER\Software\TightVNC\Server, Value: Password or PasswordViewOnly
// TigerVNC : HKEY_LOCAL_USER\Software\TigerVNC\WinVNC4, Value: Password
// UltraVNC, C:\Program Files\UltraVNC\ultravnc.ini, Value: passwd or passwd2

// ####### SSH #################
// C:\Users\{USERS}\.ssh

// ####### git #################
// C:\Users\{USERS}\.gitconfig

// ####### WSL #################
// C:\Users\{USERS}\.bash_history

// ####### CITRIX ##############
// %SystemDrive%\Documents and Settings\%username%\Application Data\Citrix\MetaFrame Password Manager

// ####### VAULT - Windows Credential Management ################
// C:\Users\[User Profile]\AppData\Local\Microsoft\Vault, C:\ProgramData\Microsoft\Vault, C:\Windows\system32\config\systemprofile\AppData\Local\Microsoft\Vault