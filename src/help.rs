pub mod help_menu { 

    use crate::enum_computer::enum_comp;
    use crate::fast_mode::fast_mode;
    use crate::all_mode::all_mode;
    use crate::vnc::vnc_check::ultra_vnc_check;

    use std::env;
    use std::process;
    use clearscreen;

    pub fn help() 
    {
        let args: Vec<String> = env::args().collect();
        
        // Checking number of arguments passed 
        match args.len() {

            1 => {
                banner();
                println!("Error ! No argument found ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files on the C: drive");
                println!("• fast : Exclude some folders for the search");
                process::exit(1);
                },

            2 => {
                match args[1].as_str() {
            
                    // If the argument is "fast"
                   "all" => 
                   { 
                    banner();
                    enum_comp::get_time();
                    enum_comp::system_information();
                    enum_comp::network_information();
                    enum_comp::powershell_version();
                    enum_comp::processes();
                    all_mode::browse_dir();
                    ultra_vnc_check();
                   },

                   // If the argument is "fast"
                   "fast" => 
                   {
                    banner();
                    enum_comp::get_time();
                    enum_comp::system_information();
                    enum_comp::network_information();
                    enum_comp::powershell_version();
                    enum_comp::processes();
                    fast_mode::browse_dir();
                    ultra_vnc_check();

                   },
                
                   // If the argument is "help"
                   "help" =>
                   {
                    banner();
                    println!("• help : Display help menu");
                    println!("• all : Search in all files on the C: drive");
                    println!("• fast : Exclude some folders for the search");
                    process::exit(1);
                   }

                   // Others cases
                   _ => {
                    banner();
                    println!("Incorrect arguments passed ! Please pass one correct argument to make it work! \n");
                    println!("• help : Display help menu");
                    println!("• all : Search in all files on the C: drive");
                    println!("• fast : Exclude some folders for the search");
                    process::exit(1);
                   },
                }
            },
            
            // If there is more than 1 argument
            _ => {
                banner();
                println!("Error ! Too much arguments passed ! \n Please pass one argument to make it work! \n");
                println!("• help : Display help menu");
                println!("• all : Search in all files on the C: drive");
                println!("• fast : Exclude some folders for the search");
                process::exit(1);
            }       
}
}

// Pure art
pub fn banner(){
    clearscreen::clear().expect("failed to clear screen");
    println!("\n  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("  @@@@@@@@@@@@@@&/////////////////////&@@@@@@@@@@@@@");
    println!("  @@@@@@@@@@%/////////////////////////////&@@@@@@@@@");
    println!("  @@@@@@@@///////////////////////////////////@@@@@@@");
    println!("  @@@@@%///////////////////////////////////////&@@@@");
    println!("  @@@@//////////////////,    ////////////////////@@@");
    println!("  @@&///////////////////      ////////////////////@@");
    println!("  @@////////////////////     *  ///////////////////@");
    println!("  @/////////////////////        .  /////////////////");
    println!("  @////////////////////// /  ,,    /   /////////////");
    println!("  ///////////////////////      /        ////////////");
    println!("  @////////////////////* /     /      //////////////");
    println!("  @//////////////////  /       /////////////////////");
    println!("  @@//////////////  *         /////////////////////@");
    println!("  @@&//////////             */////////////////////@@");
    println!("  @@@@//////////         ////////////////////////@@@");
    println!("  @@@@@////////////////////////////////////////#@@@@");
    println!("  @@@@@@@@///////////////////////////////////@@@@@@@");
    println!("  @@@@@@@@@@//////////////////////////////#@@@@@@@@@");
    println!("  @@@@@@@@@@@@@@(/////////////////////#@@@@@@@@@@@@@\n");
    println!("                        Winbean                     ");


}

}