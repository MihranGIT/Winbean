pub mod find_file { 

    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn browse_dir() {
        println!("\n • POTENTIAL INTERESTING FILES \n");
        for file in WalkDir::new("C:\\").into_iter()
                                        .filter_map(|file| file.ok()) {
            
            let path = file.path()
                                    .display();
            
            let file = file.file_name()
                                     .to_string_lossy();
            
            if (file.ends_with("password.txt") 
                || file.ends_with("pass.txt") 
                || file.ends_with("passwords.txt") 
                || file.ends_with("motdepasse.txt") 
                || file.ends_with("mdp.txt") 
                || file.ends_with("pass.txt"))
                && file.len() < 10
                {
                    println!("      Interesting file found : {}", path);
                    enum_content_file((&path)
                    .to_string());
                }

                if file.ends_with(".gitconfig")
                {
                    println!("      Git config files : {}", path);
                    enum_content_gitconfig((&path)
                    .to_string());
                }

                if file.ends_with(".bash_history")
                {
                    println!("      WSL history files found : {}", path);
                    enum_bash_history((&path)
                    .to_string());
                }

                if file.ends_with("id_rsa")
                {
                    println!("      SSH key found : {}", path);
                    enum_ssh_key((&path)
                    .to_string());
                }

                if file.ends_with(".kdbx")
                {
                    println!("      Keepas found : {}", path);
                }

                if file.ends_with(".config")
                {
                    println!("      Config file found : {}", path);
                    enum_content_config((&path)
                    .to_string())
                }
        }
    }

    pub fn enum_content_file(file: String)
    {
        let file = File::open(file.to_string())
                    .expect("Can't open the file");
        
        let reader = BufReader::new(file);
        
        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            println!("1.{} : {}", index, line);
    }
    }

    pub fn enum_content_config(file: String)
    {
        let file = File::open(file.to_string())
                        .expect("Couldn't open the file");
        let reader = BufReader::new(file);
        
        for (index, line) in reader
                                                           .lines()
                                                           .enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            
            if line.to_string()
                   .contains("password")
            {
                println!("{}. {}", index, line);
            }
    }
    }

    pub fn enum_content_gitconfig(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Couldn't open the file");
        let reader = BufReader::new(file);
        
        for (index, line) in reader
                                   .lines()
                                   .enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            if line.to_string().contains("name") 
            || line.to_string().contains("email") 
            || line.to_string().contains("password")
            {
                println!("{}. {}", index, line);
            }
        }
    }

    pub fn enum_ssh_key(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Couldn't open the file");
        let reader = BufReader::new(file);
        
        for (_index, line) in reader
                                                        .lines()
                                                        .enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            println!("{}", line);
    }
    }

    pub fn enum_bash_history(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Couldn't open the file");
        let reader = BufReader::new(file);
        
        for (index, line) in reader.lines()
                                                                 .enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            if line.to_string()
                   .contains("ssh") 
            {
                println!("{}. SSH connection found in the history : {}", index, line);
            }
        }
    }
}