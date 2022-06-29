pub mod find_file { 

    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use faccess::{AccessMode, PathExt};
    use regex::Regex;

    pub fn browse_dir() {
        println!("\n • POTENTIAL INTERESTING FILES \n");
        for file in WalkDir::new("C:\\").into_iter()
                                        .filter_map(|file| file.ok()) {
            
            let path = file.path()
            .display();

            let CheckPath = Path::new(file.path());

            let file = file.file_name()
            .to_string_lossy();
            
            if (file.ends_with("password.txt") 
                || file.ends_with("pass.txt") 
                || file.ends_with("passwords.txt") 
                || file.ends_with("motdepasse.txt") 
                || file.ends_with("mdp.txt") 
                || file.ends_with("pass.txt"))
                && file.len() < 10
                && CheckPath.readable()
                {
                    println!("      Interesting file found : {}", path);
                    enum_content_file((&path)
                    .to_string());
                }

                if file.ends_with(".gitconfig") 
                && CheckPath.readable()
                {
                    println!("      Git config files : {}", path);
                    enum_content_gitconfig((&path)
                    .to_string());
                }

                if file.ends_with(".bash_history") 
                && CheckPath.readable()
                {
                    println!("      WSL history files found : {}", path);
                    enum_bash_history((&path)
                    .to_string());
                }

                if file.ends_with("id_rsa") 
                && CheckPath.readable()
                {
                    println!("      SSH key found : {}", path);
                    enum_ssh_key((&path)
                    .to_string());
                }

                if file.ends_with(".kdbx") 
                && CheckPath.readable()
                {
                    println!("      Keepas found : {}", path);
                }

                if file.ends_with(".config") 
                && CheckPath.readable()
                {
                    println!("      Config file found : {}", path);
                    enum_content_config((&path)
                    .to_string())
                }
                if file.ends_with(".txt") 
                && CheckPath.readable() 
                && file.len() < 10
                {
                    println!("      Text file found : {}", path);
                    enum_txt_file((&path)
                    .to_string())
                }
                if (file.ends_with(".psd1") 
                || file.ends_with(".ps1") 
                || file.ends_with(".psm1") 
                || file.ends_with(".bat")) 
                && CheckPath.readable() && file.len() < 10
                {
                    println!("      Script file found : {}", path);                                        
                    enum_script(((&path))
                    .to_string())
                }
        }
    }

    pub fn enum_content_file(file: String)
    {
        let file = File::open(file.to_string());
                    //.expect("Can't open the file");
                    let file = match file 
                    {
                        Ok(file) => file,
                        Err(e) => panic!("Can't open the file"),
                    };
        let reader = BufReader::new(file);
        
        for (index, line) in reader.lines()
        .enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            println!("            {} : {}", index, line);
        }
    }

    pub fn enum_content_config(file: String)
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
                   .contains("password")
            {
                println!("            {}. {}", index, line);
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
                println!("            {}. {}", index, line);
            }
        }
    }

    pub fn enum_ssh_key(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Couldn't open the file");
        let reader = BufReader::new(file);
        
        for (_index, line) in reader.lines()
        .enumerate() 
        {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
            };
            println!("            {}", line);
        }
    }

    pub fn enum_txt_file(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Error happened while trying to read the text file !");
        let reader = BufReader::new(file);
        
        for (index, line) in reader.lines()
        .enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(error) => continue,
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
        let file = File::open(file.to_string())
                .expect("Error happened while trying to read the script !");
        let reader = BufReader::new(file);

        for (index, line) in reader.lines()
        .enumerate() 
        {
            let line = match line 
            {
            Ok(line) => line,
            Err(error) => continue,
            };
            println!("            {}. {}", index, line);
        }
    }

    pub fn enum_bash_history(file: String)
    {
        let file = File::open(file.to_string())
                                  .expect("Error happened while reading the bash history !");
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
                println!("            {}. SSH connection found in the history : {}", index, line);
            }
        }
    }
}