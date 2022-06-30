pub mod help_menu { 

    use crate::enum_computer::enum_comp;
    use crate::file_finder::find_file;
    use crate::faster::fast;

    use std::env;
    use std::process;
    use clearscreen;

    pub fn help() 
    {
        let args: Vec<String> = env::args()
        .collect();

    // Check arguments lenghts
        match args.len() {

            1 => {
                banner();
                println!("Error ! No argument found ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files and display all files");
                process::exit(1);
                },

            2 => {
                match args[1].as_str() {
            
                   "all" => 
                   { 
                    banner();
                    enum_comp::get_time();
                    enum_comp::system_information();
                    enum_comp::network_information();
                    enum_comp::powershell_version();
                    enum_comp::processes();
                    find_file::browse_dir();
                   },

                   "fast" => 
                   {
                    banner();
                    enum_comp::get_time();
                    enum_comp::system_information();
                    enum_comp::network_information();
                    enum_comp::powershell_version();
                    enum_comp::processes();
                    fast::browse_dir();

                   },

                   _ => {
                    banner();
                    println!("Incorrect arguments passed !");
                    process::exit(1);
                   },
                }
            },
            
            // Others cases
            _ => {
                banner();
                println!("Error ! Too much arguments passed ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files and display all files");
                process::exit(1);
            }       
}
}

// Banner
pub fn banner(){
    clearscreen::clear().expect("failed to clear screen");
    println!("#################################################################################################");
    println!("#####                                 WinBean - v1.0                                        #####");
    println!("################################################################################################# \n \n");
}

}