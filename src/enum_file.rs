pub mod enum_file {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use regex::Regex;
    use lazy_static::lazy_static;

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

    pub fn enum_content_gitconfig(file: String, gitconfig_file: &mut Vec<String>)
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
                gitconfig_file.push(line.to_string());
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

    pub fn enum_script(file: String, _script_file: &mut Vec<String>)
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
}