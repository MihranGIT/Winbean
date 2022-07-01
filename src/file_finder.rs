pub mod find_file { 

    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use faccess::{PathExt};

    // Browse all the directories on the machine
    pub fn browse_dir() {
        
        println!("\n • POTENTIAL INTERESTING FILES \n");
        for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
            let path = file.path().display();
            let check_path = Path::new(file.path());
            let file = file.file_name().to_string_lossy();
            
                if (file.to_lowercase().ends_with("password.txt") || file.to_lowercase().ends_with("pass.txt") || file.to_lowercase().ends_with("passwords.txt") || file.to_lowercase().ends_with("motdepasse.txt") || file.to_lowercase().ends_with("mdp.txt") || file.to_lowercase().ends_with("pass.txt"))
                && file.len() < 10
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