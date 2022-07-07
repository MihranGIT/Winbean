pub mod ssh_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
}