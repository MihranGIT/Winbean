pub mod txt_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // Looking in text files for any mention of passwords
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
            if line.to_string().to_lowercase().contains("password")
            {
                println!("            > Line {} - \"{}\"", index, line);
                txt_file.push(line.to_string());
            }
        }
    }
}