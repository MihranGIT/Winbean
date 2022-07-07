pub mod localtime_enum {

    use chrono::prelude::*;

    pub fn get_time()
    {
        let local: DateTime<Local> = Local::now();
        println!("\n • LOCAL TIME \n");
        println!("      {}", local);
    }
}