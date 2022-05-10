pub mod find_file { 

    use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, HKEY_USERS, KEY_READ};
    use whoami;
    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::io::Write;

    pub fn browse_dir() {
        println!("==================================================================");
        for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
            let path = file.path().display();
            let file = file.file_name().to_string_lossy();
            
            if (file.ends_with("\\Desktop\\password.txt") || file.ends_with("pass.txt") || file.ends_with("passwords.txt") 
            || file.ends_with("motdepasse.txt") || file.ends_with("mdp.txt") || file.ends_with("pass.txt"))
            && file.len() < 10
            {
                println!("Potentiel fichier interessant trouve : {}", path);
                enum_content_file((&path).to_string());
            }
            if file.ends_with(".gitconfig")
            {
                println!("Fichier config git : {}", path);
                enum_content_gitconfig((&path).to_string());
            }
            if file.ends_with(".bash_history")
            {
                println!("Fichier d'historique WSL trouve : {}", path);
                enum_bash_history((&path).to_string());
            }
            if file.ends_with("id_rsa")
            {
                println!("Fichier ssh trouve : {}", path);
                enum_ssh_key((&path).to_string());
            }
            if file.ends_with(".kdbx")
            {
                println!("Fichier KeePass trouve : {}", path);
            }
            if file.ends_with(".config")
            {
                println!("Fichier de conférence trouve : {}", path);
                enum_content_config((&path).to_string())
            }
        }
    }

    pub fn enum_content_file(file: String)
    {
        let file = File::open(file.to_string()).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            println!("1.{} : {}", index, line);
    }
    }

    pub fn enum_content_config(file: String)
    {
        let file = File::open(file.to_string()).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.to_string().contains("password"){
                println!("{}. {}", index, line);
            }
    }
    }

    pub fn enum_content_gitconfig(file: String)
    {
        let file = File::open(file.to_string()).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.to_string().contains("name") 
            || line.to_string().contains("email") 
            || line.to_string().contains("password"){
                println!("{}. {}", index, line);
            }
    }
    }

    pub fn enum_ssh_key(file: String)
    {
        let file = File::open(file.to_string()).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            println!("{}", line);
    }
    }

    pub fn enum_bash_history(file: String)
    {
        let file = File::open(file.to_string()).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.to_string().contains("ssh") {
                println!("{}. SSH connection : {}", index, line);
            }
    }
    }
}