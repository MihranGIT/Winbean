pub mod all_mode {

    use walkdir::WalkDir;
    use std::path::Path;
    use faccess::{PathExt};
    use crate::enum_file::enum_file;
    use crate::print_vector::print_vector;

    pub fn browse_dir() {

            let mut passwords = Vec::new();
            let mut interresting_file = Vec::new();
            let mut gitconfig_file = Vec::new();
            let mut bash_history_file = Vec::new();
            let mut ssh_key = Vec::new();
            //let mut keypass_file = Vec::new();
            let mut script_file = Vec::new();
            let mut txt_file = Vec::new();
            let mut config_file = Vec::new();
            
            println!("\n • POTENTIAL INTERESTING FILES \n");
            
            // Searching through C:\ drive only
            for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
                let path = file.path().display();
                let check_path = Path::new(file.path());
                let file = file.file_name().to_string_lossy();
                

                    // Filtering per filename, to_lowercase() to avoid missing some file like "Password.txt", "PassWORD.txt" etc...
                    // Filtering per extension (txt file) and checking if the file is readable to prevent crash
                    if (file.to_lowercase().contains("password") || file.to_lowercase().contains("pass") || file.to_lowercase().contains("passwords") || file.to_lowercase().contains("motdepasse") || file.to_lowercase().contains("mdp") || file.to_lowercase().contains("pass"))
                    && file.len() < 10
                    && file.ends_with(".txt")
                    && check_path.readable()
                    {
                        println!("      Interesting file found : {}", path);
                        interresting_file.push(path.to_string());
                        enum_file::enum_content_file((&path).to_string());
                    }

                    // Filtering gitconfig file
                    if file.ends_with(".gitconfig") 
                    && check_path.readable()
                    {
                        println!("      Git config files : {}", path);
                        enum_file::enum_content_gitconfig((&path).to_string(), &mut gitconfig_file);
                    }

                    // Looking for bash_history (with WSL for example) on the machine
                    if file.ends_with(".bash_history") 
                    && check_path.readable()
                    {
                        println!("      WSL history files found : {}", path);
                        enum_file::enum_bash_history((&path).to_string(), &mut bash_history_file);
                    }

                    // Looking for SSH key based on the filename
                    if (file.ends_with("id_rsa")|| file.ends_with("id_dsa")|| file.ends_with("id_ed25519") || file.ends_with("id_ecdsa") || file.ends_with(".ppk"))
                    && check_path.readable()
                    {
                        println!("      SSH key found : {}", path);
                        enum_file::enum_ssh_key((&path).to_string(), &mut ssh_key);
                    }

                    // Looking for KeePass file
                    if file.ends_with(".kdbx") 
                    && check_path.readable()
                    {
                        println!("      Keepass found : {}", path);
                        // keypass_file.push(&path.to_string());
                    }

                    // Looking for .config file
                    if file.ends_with(".config") 
                    && check_path.readable()
                    {
                        enum_file::enum_content_config((&path).to_string(), &mut config_file);
                    }
                    
                    // Looking specificly for web.config file (default filename for IIS server config file)
                    if file.ends_with("\\web.config")
                    && check_path.readable()
                    {
                        println!("      Possible IIS Server : web.config file found ({})", &path.to_string());
                    }
                    
                    // Looking for txt file < 10ko
                    if file.ends_with(".txt") 
                    && check_path.readable() 
                    && file.len() < 10
                    {
                        enum_file::enum_txt_file((&path).to_string(), &mut txt_file);
                    }

                    // Looking for txt file on the Desktop
                    if file.ends_with(".txt")
                    && check_path.readable()
                    && path.to_string().contains("\\Desktop\\")
                    && file.len() < 10
                    {
                        println!("      Found txt file on the Desktop: {}, checking If it contains a password", path);
                        enum_file::enum_txt_deskop_file((&path).to_string(), &mut passwords);
                    }
                    
                    // Looking for script file (bat and powershell)
                    if (file.ends_with(".psd1") || file.ends_with(".ps1") || file.ends_with(".psm1") || file.ends_with(".bat")) 
                    && check_path.readable() 
                    && file.len() < 10
                    {
                        println!("      Script file found : {}", path);                                        
                        enum_file::enum_script(((&path)).to_string(), &mut script_file);
                        script_file.push(path.to_string());
                    }
                }
                    // Sort and remove all duplicates from the vectors
        passwords.sort();
        passwords.dedup();
        bash_history_file.sort();
        bash_history_file.dedup();

        //print_vector is a function to print one element of a vector per line
        print_vector::print_vector(&script_file);
        println!("\n\n      • Bash history file found : \n");
        print_vector::print_vector(&bash_history_file);
        println!("\n\n      • Interesting file found : \n");
        print_vector::print_vector(&interresting_file);
        println!("\n\n      • Git config file found : \n");
        print_vector::print_vector(&gitconfig_file);
        println!("\n\n      • Config file found : \n");
        print_vector::print_vector(&config_file);
        println!("\n\n      • Potential SSH key found \n");
        print_vector::print_vector(&ssh_key);
        println!("\n\n      • Potential passwords found\n");
        print_vector::print_vector(&passwords);
        }
}
