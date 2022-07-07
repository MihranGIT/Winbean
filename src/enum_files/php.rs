pub mod php_enum { 

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    pub fn enum_php_file(file: String, php_credentials: &mut Vec<String>){
        let file = File::open(file.to_string()).expect("Error happened while reading the bash history !");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = match line 
            {
                Ok(line) => line,
                Err(_error) => continue,
            };
            if line.to_string().contains("cubrid_connect(") || line.to_string().contains("ibase_pconnect(") || line.to_string().contains("fbird_pconnect(") || line.to_string().contains("db2_pconnect(") ||
            line.to_string().contains("mysqli::real_connect(") || line.to_string().contains("mysql_connect(") || line.to_string().contains("oci_connect(") || line.to_string().contains("pg_connect(") || 
            line.to_string().contains("sqlsrv_connect(") || line.to_string().contains("curl_setopt(") ||
            line.to_string().contains("pass")
            {
                println!("            > Line {} - Found : {}", index, line);
                php_credentials.push(line.to_string());
            }
        }
    }
}