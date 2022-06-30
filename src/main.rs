use crate::enum_computer::enum_comp;
use crate::file_finder::find_file;
use crate::help::help_menu;
use std::time::{Duration, Instant};

mod help;
mod enum_computer;
mod file_finder;

fn main() 
{
    let start = Instant::now();
    // Help and parsing arguments
    help_menu::help();
    println!("\n Finished ! Checked done in {:?}", start.elapsed());
}