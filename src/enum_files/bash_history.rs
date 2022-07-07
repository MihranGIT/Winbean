pub mod bash_history_enum {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
}