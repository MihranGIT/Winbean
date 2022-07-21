pub mod scripts_enum { 

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // Looking through windows script files (Batch and Powershell) any argument "-p", "ssh", "mysql" or "password" in the file
    // Trying to filter false positive by eliminating comment, who starts by Rem (Batch) or # (PowerShell)
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
            if (line.to_string().contains("password") || line.to_string().contains("-SecureString") || line.to_string().contains("-AsPlainText") || line.to_string().contains("-p") 
            || line.to_string().contains("ssh") || line.to_string().contains("mysql")) || line.to_string().contains("\\[Net.NetworkCredential\\]::new\\(") || line.to_string().contains("net user")
            || line.to_string().contains("cmdkey")
            && line.to_string().contains("Rem") == false && line.to_string().contains("#")
            {
                println!("            > Line {} - \"{}\"", index, line);
            }           
        }
    }
}