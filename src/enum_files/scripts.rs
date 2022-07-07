pub mod scripts_enum { 

    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
            if (line.to_string().contains("password") || line.to_string().contains("-p") || line.to_string().contains("ssh") || line.to_string().contains("mysql")) && line.to_string().contains("Rem") == false && line.to_string().contains("#")
            {
                println!("            > Line {} - \"{}\"", index, line);
            }           
        }
    }
}