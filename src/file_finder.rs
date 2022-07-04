pub mod find_file { 

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
        for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
            let path = file.path().display();
            let check_path = Path::new(file.path());
            let file = file.file_name().to_string_lossy();
            
                if (file.to_lowercase().contains("password") || file.to_lowercase().contains("pass") || file.to_lowercase().contains("passwords") || file.to_lowercase().contains("motdepasse") || file.to_lowercase().contains("mdp") || file.to_lowercase().contains("pass"))
                && file.len() < 10
                && file.ends_with(".txt")
                && check_path.readable()
                {
                    println!("      Interesting file found : {}", path);
                    enum_content_file((&path).to_string(), &mut interresting_file);
                }

                if file.ends_with(".gitconfig") 
                && check_path.readable()
                {
                    println!("      Git config files : {}", path);
                    enum_content_gitconfig((&path).to_string(), &mut gitconfig_file);
                }

                if file.ends_with(".bash_history") 
                && check_path.readable()
                {
                    println!("      WSL history files found : {}", path);
                    enum_bash_history((&path).to_string(), &mut bash_history_file);
                }

                if (file.ends_with("id_rsa")|| file.ends_with("id_dsa")|| file.ends_with("id_ed25519"))
                && check_path.readable()
                {
                    println!("      SSH key found : {}", path);
                    enum_ssh_key((&path).to_string(), &mut ssh_key);
                }

                if file.ends_with(".kdbx") 
                && check_path.readable()
                {
                    println!("      Keepas found : {}", path);
                    // keypass_file.push(&path.to_string());
                }

                if file.ends_with(".config") 
                && check_path.readable()
                {
                    enum_content_config((&path).to_string(), &mut config_file);
                }
                
                if file.ends_with(".txt") 
                && check_path.readable() 
                && file.len() < 10
                {
                    enum_txt_file((&path).to_string(), &mut txt_file);
                }

                if file.ends_with(".txt")
                && check_path.readable()
                && path.to_string().contains("Desktop")
                && file.len() < 10
                {
                    println!("Found txt file on the Desktop: {}, checking If it contains a password", path);
                    enum_txt_deskop_file((&path).to_string(), &mut passwords);
                }
                
                if (file.ends_with(".psd1") || file.ends_with(".ps1") || file.ends_with(".psm1") || file.ends_with(".bat")) 
                && check_path.readable() 
                && file.len() < 10
                {
                    println!("      Script file found : {}", path);                                        
                    enum_script(((&path)).to_string(), &mut script_file);
                }
        }
        println!("Result :");
        println!("Passwords found : {:?}", passwords);
        println!("Interesting file found : {:?}", interresting_file);
        println!("Git config file found : {:?}", gitconfig_file);
        println!("Bash history file found : {:?}", bash_history_file);
    }

    pub fn enum_content_file(file: String, interresting_file: &mut Vec<String>)
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
            println!("            {} : {}", index, line);
            interresting_file.push(line.to_string());
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
                println!("            {}. {}", index, line);
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
                println!("            {}. {}", index, line);
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
                println!("            {}. {}", index, line);
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
            if line.to_string().contains("password") || line.to_string().contains("-p") || line.to_string().contains("host") || line.to_string().contains("ssh")
            {
                println!("            {}. {}", index, line);
                script_file.push(line.to_string());
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
                println!("            {}. SSH connection found in the history : {}", index, line);
                bash_history_file.push(line.to_string());
            }
        }
    }

    pub fn enum_txt_deskop_file(file: String, passwords: &mut Vec<String>)
    {
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
                if word.len() > 7 {
                    if RE.is_match(word){
                        println!("Potential password found : {}", word);
                        passwords.push(word.to_string());
                }
            }
            }
        }
    }
}