use crate::help::help_menu;
use std::time::{Instant};

mod help;
mod enum_computer;
mod file_finder;
mod faster;

fn main() 
{
    let start = Instant::now();
    // Help and parsing arguments
    help_menu::help();
    println!("\n Finished ! Checked done in {:?}", start.elapsed());
}