pub mod gitconfig_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // Looking for name, emails and passwords stocked on the gitconfig files
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
}