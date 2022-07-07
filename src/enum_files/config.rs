pub mod config_enum {    

    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
            
            if line.to_string().to_lowercase().contains("password")
            {
                println!("            > Line {} - \"{}\"", index, line);
                config_file.push(line.to_string());          
            }
        }
    }
}