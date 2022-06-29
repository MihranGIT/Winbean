pub mod help_menu { 
    
    use std::env;
    use std::process;
    use clearscreen;

    pub fn help() 
    {
        let args: Vec<String> = env::args().collect();

        match args.len() {

            1 => {
                banner();
                println!("Error ! No argument found ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files and display all files");
                process::exit(1);
                },

            2 => {
                match args[1].parse() {
                    Ok(42) => println!("This is the answer!"),
                    _ => println!("This is not the answer."),
                }
            },
            
            _ => {
                banner();
                println!("Error ! Too much arguments passed ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files and display all files");
                process::exit(1);
            }       
}
}

pub fn banner(){
    clearscreen::clear().expect("failed to clear screen");
    println!("#################################################################################################");
    println!("#####                                 WinBean - v1.0                                        #####");
    println!("################################################################################################# \n \n");
}

}