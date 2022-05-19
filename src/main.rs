use crate::enum_computer::enum_comp;
use crate::file_finder::find_file;

mod enum_computer;
mod file_finder;

fn main() 
{
    enum_comp::get_time();
    enum_comp::system_information();
    enum_comp::network_information();
    enum_comp::powershell_version();
    enum_comp::processes();
    find_file::browse_dir();
}


