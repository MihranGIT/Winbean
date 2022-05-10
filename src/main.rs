use crate::enum_computer::enum_comp;
use crate::file_finder::find_file;

mod enum_computer;
mod file_finder;

fn main() 
{
    enum_computer::enum_comp::get_time();
    enum_computer::enum_comp::system_information();
    enum_computer::enum_comp::network_information();
    enum_computer::enum_comp::powershell_version();
    enum_computer::enum_comp::processes();
    file_finder::find_file::browse_dir();
}


