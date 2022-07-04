pub mod fast {
    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use faccess::{PathExt};
    use regex::Regex;
    use lazy_static::lazy_static;

    // Browse all the directories on the machine
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
            
            // Filter all these folders 
            if path.to_string().contains("WindowsPowerShell\\Modules") || path.to_string().contains("syswow64") || path.to_string().contains("system32") || path.to_string().contains("windows\\servicing") || path.to_string().contains("servicing") 
            || path.to_string().contains("\\Microsoft\\.NET\\Framework") || path.to_string().contains("Windows") || path.to_string().contains("wsuscontent") || path.to_string().contains("\\puppet\\share\\doc") || path.to_string().contains("\\lib\\ruby")
            || path.to_string().contains("\\lib\\site-packages") || path.to_string().contains("\\usr\\share\\doc") || path.to_string().contains("node_modules") || path.to_string().contains("vendor\\bundle") || path.to_string().contains("vendor\\cache") 
            || path.to_string().contains("\\doc\\openssl") || path.to_string().contains("Python27\\Lib") {
                continue
            }
            else {

                // Filtering per filename, to_lowercase() to avoid missing some file like "Password.txt", "PassWORD.txt" etc...
                // Filtering per extension (txt file) and checking if the file is readable to prevent crash
                if (file.to_lowercase().contains("password") || file.to_lowercase().contains("pass") || file.to_lowercase().contains("passwords") || file.to_lowercase().contains("motdepasse") || file.to_lowercase().contains("mdp") || file.to_lowercase().contains("pass"))
                && file.len() < 10
                && file.ends_with(".txt")
                && check_path.readable()
                {
                    println!("      Interesting file found : {}", path);
                    interresting_file.push(path.to_string());
                    enum_content_file((&path).to_string());
                }

                // Filtering gitconfig file
                if file.ends_with(".gitconfig") 
                && check_path.readable()
                {
                    println!("      Git config files : {}", path);
                    enum_content_gitconfig((&path).to_string(), &mut gitconfig_file);
                }

                // Looking for bash_history (with WSL for example) on the machine
                if file.ends_with(".bash_history") 
                && check_path.readable()
                {
                    println!("      WSL history files found : {}", path);
                    enum_bash_history((&path).to_string(), &mut bash_history_file);
                }

                // Looking for SSH key based on the filename
                if (file.ends_with("id_rsa")|| file.ends_with("id_dsa")|| file.ends_with("id_ed25519"))
                && check_path.readable()
                {
                    println!("      SSH key found : {}", path);
                    enum_ssh_key((&path).to_string(), &mut ssh_key);
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
                    enum_content_config((&path).to_string(), &mut config_file);
                }
                
                // Looking specificly for web.config file (default filename for IIS server config file)
                if file.ends_with("web.config")
                && check_path.readable()
                {
                    println!("      Possible IIS Server : web.config file found ({})", &path.to_string());
                }
                
                // Looking for txt file < 10ko
                if file.ends_with(".txt") 
                && check_path.readable() 
                && file.len() < 10
                {
                    enum_txt_file((&path).to_string(), &mut txt_file);
                }

                // Looking for txt file on the Desktop
                if file.ends_with(".txt")
                && check_path.readable()
                && path.to_string().contains("\\Desktop\\")
                && file.len() < 10
                {
                    println!("      Found txt file on the Desktop: {}, checking If it contains a password", path);
                    enum_txt_deskop_file((&path).to_string(), &mut passwords);
                }
                
                // Looking for script file (bat and powershell)
                if (file.ends_with(".psd1") || file.ends_with(".ps1") || file.ends_with(".psm1") || file.ends_with(".bat")) 
                && check_path.readable() 
                && file.len() < 10
                {
                    println!("      Script file found : {}", path);                                        
                    enum_script(((&path)).to_string(), &mut script_file);
                    script_file.push(path.to_string());
                }
        }
    }
        println!("      \n\nDone ! Resume :\n");
        println!("      • Script file found : \n");

        // Sort and remove all duplicates from the vectors
        passwords.sort();
        passwords.dedup();
        bash_history_file.sort();
        bash_history_file.dedup();

        //print_vector is a function to print one element of a vector per line
        print_vector(&script_file);
        println!("\n\n      • Bash history file found : \n");
        print_vector(&bash_history_file);
        println!("\n\n      • Interesting file found : \n");
        print_vector(&interresting_file);
        println!("\n\n      • Git config file found : \n");
        print_vector(&gitconfig_file);
        println!("\n\n      • Config file found : \n");
        print_vector(&config_file);
        println!("\n\n      • Potential SSH key found \n");
        print_vector(&ssh_key);
        println!("\n\n      • Potential passwords found\n");
        print_vector(&passwords);
    
    }

    pub fn enum_content_file(file: String)
    {
        let file = File::open(file.to_string()).expect("Couldn't open the file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            println!("            Potential password found : Line {} : \"{}\"", index, line);
        }
    }

    pub fn enum_content_config(file: String, config_file: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Couldn't open the file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            
            if line.to_string().contains("password")
            {
                println!("            > Line {} - \"{}\"", index, line);
                config_file.push(line.to_string());          
            }
    }
    }

    pub fn enum_content_gitconfig(file: String, ssh_key: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Couldn't open the file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            if line.to_string().contains("name") || line.to_string().contains("email") || line.to_string().contains("password")
            {
                println!("            > Line {} - \"{}\"", index, line);
                ssh_key.push(line.to_string());
            }
        }
    }

    pub fn enum_ssh_key(file: String, ssh_key: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Couldn't open the file");
        let reader = BufReader::new(file);
        for (_index, line) in reader.lines().enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            println!("            {}", line);
            ssh_key.push(line.to_string());
        }
    }

    pub fn enum_txt_file(file: String, txt_file: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Error happened while trying to read the text file !");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            if line.to_string()
                   .contains("password")
            {
                println!("            > Line {} - \"{}\"", index, line);
                txt_file.push(line.to_string());
            }
        }
    }

    pub fn enum_script(file: String, script_file: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Error happened while trying to read the script !");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() 
        {
            let line = match line 
            {
            Ok(line) => line,
            Err(_error) => continue,
            };
            if line.to_string().contains("password") || line.to_string().contains("-p") || line.to_string().contains("host") || line.to_string().contains("ssh") || line.to_string().contains("mysql")
            {
                println!("            > Line {} - \"{}\"", index, line);
            }           
        }
    }

    pub fn enum_bash_history(file: String, bash_history_file: &mut Vec<String>)
    {
        let file = File::open(file.to_string()).expect("Error happened while reading the bash history !");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            if line.to_string().contains("ssh") 
            {
                println!("            > Line {} - SSH connection found in the history : {}", index, line);
                bash_history_file.push(line.to_string());
            }
        }
    }

    pub fn enum_txt_deskop_file(file: String, passwords: &mut Vec<String>)
    {
        // Using lazy_static to ensure that regular expressions are compiled exactly once
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w*[a-z]\w*[0-9]").expect("Error with the regex");
        }
        let file = File::open(file.to_string()).expect("Error happened while trying to read the text file !");
        let reader = BufReader::new(file);

        for (_index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            for word in line.split_whitespace() {
                if word.len() > 7 && RE.is_match(word) && word.starts_with("http") == false && word.starts_with("www") == false && word.starts_with("C:\\") == false {
                        println!("           > Potential password found : {}", word);
                        passwords.push(word.to_string());
                }
            }
        }
    }

    // Function to display 1 element of a vector line per line 
    pub fn print_vector(vec: &Vec <String>){
        for word in vec.iter(){
            println!("{}", word);
        }
    }
}
