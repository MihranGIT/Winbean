pub mod regex_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use regex::Regex;
    use lazy_static::lazy_static;
    
    // Looking on txt files on the user desktop (typically where they stored their passwords)
    // A regex is used (8 characters with DIGIT and ALPHA) to try getting a password
    pub fn enum_txt_deskop_file(file: String, passwords: &mut Vec<String>, fileshare: &mut Vec<String>, url: &mut Vec<String>)
    {
        // Using lazy_static to ensure that regular expressions are compiled exactly once
        lazy_static! {
            static ref MDP: Regex = Regex::new(r"\w*[a-z]\w*[0-9]").expect("Error with the regex");
            static ref FILESHARE: Regex = Regex::new(r"^(\\)(\\[\w\.-_]+){2,}(\\?)$").expect("Error with the regex");
            static ref URL: Regex = Regex::new(r"(https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|www\.[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9]+\.[^\s]{2,}|www\.[a-zA-Z0-9]+\.[^\s]{2,})").expect("Error with the regex");
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
                if word.len() > 7 && word.len() < 30 && MDP.is_match(word) && word.starts_with("http") == false && word.starts_with("www") == false && word.starts_with("C:\\") == false {
                    println!("           > Potential password found : {}", word);
                    passwords.push(word.to_string());
                }
                if FILESHARE.is_match(word) {
                    println!("           > Potential fileshare found : {}", word);
                    fileshare.push(word.to_string());
                }
                if word.to_lowercase().starts_with("http") || word.to_lowercase().starts_with("www") {
                    println!("           > Potential url found : {}", word);
                    url.push(word.to_string());
                }
            }
        }
    }
}