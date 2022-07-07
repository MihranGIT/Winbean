pub mod help_menu { 

    use crate::enum_computer::enum_comp;
    use crate::fast_mode::fast_mode;
    use crate::all_mode::all_mode;

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
                    enum_comp::enumerate_all();
                    all_mode::browse_dir(&String::from("C:\\"));
                   },

                   // If the argument is "fast"
                   "fast" => 
                   {
                    banner();
                    enum_comp::enumerate_all();
                    fast_mode::browse_dir(&String::from("C:\\"));

                   },
                
                   // If the argument is "help"
                   "help" =>
                   {
                    banner();
                    println!("\nWinbean is a post-exploitation tool harvesting credentials on the machine. It's looking through files based on pattern, file extension, filename, specific words or function used. \nIt is not looking only for passwords but also files that could be interesting like scripts files, txt files, etc... \n");
                    println!("Basic commands :");
                    println!("• help : Display help menu");
                    println!("• all : Specific mode to search in all files on the drive (by default on the C:\\ drive, specify for another letter or a specific folder)");
                    println!("• fast : Specific mode to exclude some folders for the search, faster than the all mode (by default on C:\\ drive, specify for another letter or a specific folder).\n");
                    println!("Command : \"winbean.exe <mode> <drive or folder>\"");
                    println!("Example : \"winbean.exe fast D:\\\" (fast mode harvesting on the D:\\ drive)");
                    println!("          \"winbean.exe all C:\\Users\" (all mode harvesting starting on C:\\Users)"); 
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

            3 => {   
                match args[1].as_str() {

                "all" => 
                   { 
                    banner();
                    enum_comp::enumerate_all();
                    all_mode::browse_dir(&String::from(args[2].as_str()));
                   },

                   // If the argument is "fast"
                   "fast" => 
                   {
                    banner();
                    enum_comp::enumerate_all();
                    fast_mode::browse_dir(&String::from(args[2].as_str()));

                   },

                   _ => {
                    banner();
                    println!("Incorrect arguments passed ! Please pass one correct argument to make it work! \n");
                    println!("• help : Display help menu");
                    println!("• all : Search in all files on the C: drive");
                    println!("• fast : Exclude some folders for the search");
                    process::exit(1);
                   },
            }
        }

            
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