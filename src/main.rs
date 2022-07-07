use crate::help::help_menu;
use std::time::{Instant};

mod help;
mod enum_computer;
mod enum_file;
mod all_mode;
mod fast_mode;
mod print_vector;

fn main() 
{
    let start = Instant::now();
    help_menu::help();
    // Displaying time elapsed during the execution
    println!("\n Finished ! Checked done in {:?}", start.elapsed());
}