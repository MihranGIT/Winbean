pub mod vnc_check {

    use std::path::Path;

    pub fn ultra_vnc_check() {
        let ultra_vnc = String::from("C:\\Program Files\\UltraVNC\\ultravnc.ini");
        if Path::new(&ultra_vnc).exists() == true {
            println!("\nUltraVNC password file : Found in {}", ultra_vnc);
        }
        else {
            println!("\nUltraVNC password file : Not found on the default location");
        }
    }
}