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
        
        println!("\n • POTENTIAL INTERESTING FILES \n");
        for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
            let path = file.path().display();
            let check_path = Path::new(file.path());
            let file = file.file_name().to_string_lossy();
            
            if path.to_string().contains("WindowsPowerShell\\Modules") || path.to_string().contains("syswow64") || path.to_string().contains("system32") || path.to_string().contains("windows\\servicing") || path.to_string().contains("servicing") 
            || path.to_string().contains("\\Microsoft\\.NET\\Framework") || path.to_string().contains("Windows") || path.to_string().contains("wsuscontent") || path.to_string().contains("\\puppet\\share\\doc") || path.to_string().contains("\\lib\\ruby")
            || path.to_string().contains("\\lib\\site-packages") || path.to_string().contains("\\usr\\share\\doc") || path.to_string().contains("node_modules") || path.to_string().contains("vendor\\bundle") || path.to_string().contains("vendor\\cache") 
            || path.to_string().contains("\\doc\\openssl") || path.to_string().contains("Python27\\Lib") {
                continue
            }
            else {
                if (file.to_lowercase().contains("password") || file.to_lowercase().contains("pass") || file.to_lowercase().contains("passwords") || file.to_lowercase().contains("motdepasse") || file.to_lowercase().contains("mdp") || file.to_lowercase().contains("pass"))
                && file.len() < 10
                && file.ends_with(".txt")
                && check_path.readable()
                {
                    println!("      Interesting file found : {}", path);
                    enum_content_file((&path).to_string());
                }

                if file.ends_with(".gitconfig") 
                && check_path.readable()
                {
                    println!("      Git config files : {}", path);
                    enum_content_gitconfig((&path).to_string());
                }

                if file.ends_with(".bash_history") 
                && check_path.readable()
                {
                    println!("      WSL history files found : {}", path);
                    enum_bash_history((&path).to_string());
                }

                if (file.ends_with("id_rsa")|| file.ends_with("id_dsa")|| file.ends_with("id_ed25519"))
                && check_path.readable()
                {
                    println!("      SSH key found : {}", path);
                    enum_ssh_key((&path).to_string());
                }

                if file.ends_with(".kdbx") 
                && check_path.readable()
                {
                    println!("      Keepas found : {}", path);
                }

                if file.ends_with(".config") 
                && check_path.readable()
                {
                    enum_content_config((&path).to_string())
                }
                
                if file.ends_with(".txt") 
                && check_path.readable() 
                && file.len() < 10
                {
                    enum_txt_file((&path).to_string())
                }
                if file.ends_with(".txt")
                && check_path.readable()
                && path.to_string().contains("Desktop")
                && file.len() < 10
                {
                    println!("Found txt file on the Desktop: {}, checking If it contains a password", path);
                    enum_txt_deskop_file((&path).to_string());
                }

                if (file.ends_with(".psd1") || file.ends_with(".ps1") || file.ends_with(".psm1") || file.ends_with(".bat")) 
                && check_path.readable() 
                && file.len() < 10
                {
                    println!("      Script file found : {}", path);                                        
                    enum_script(((&path)).to_string())
                }
        }
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
            println!("            {} : {}", index, line);
        }
    }

    pub fn enum_content_config(file: String)
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
            }
    }
    }

    pub fn enum_content_gitconfig(file: String)
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
            }
        }
    }

    pub fn enum_ssh_key(file: String)
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
        }
    }

    pub fn enum_txt_deskop_file(file: String)
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w*[a-z]\w*[0-9]").expect("Error with the regex");
        }
        let file = File::open(file.to_string()).expect("Error happened while trying to read the text file !");
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            for word in line.split_whitespace() {
                if word.len() > 7 {
                    if RE.is_match(word){
                        println!("Potential password found : {}", word);
                }
            }
            }
        }
    }
    
    pub fn enum_txt_file(file: String)
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
            }
        }
    }

    pub fn enum_script(file: String)
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
            }           
        }
    }

    pub fn enum_bash_history(file: String)
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
            }
        }
    }
    }
}