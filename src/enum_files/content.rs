pub mod content_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // Enumerating content file of txt files with "pass" or "password" in the filename
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
}