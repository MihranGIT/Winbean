use crate::enum_computer::enum_comp;
use crate::file_finder::find_file;
use crate::help::help_menu;

mod help;
mod enum_computer;
mod file_finder;

fn main() 
{
    // Help and parsing arguments
    help_menu::help();

    // Enum functions
    enum_comp::get_time();
    enum_comp::system_information();
    enum_comp::network_information();
    enum_comp::powershell_version();
    enum_comp::processes();

    // File finder
    find_file::browse_dir();
}


