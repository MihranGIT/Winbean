pub mod processes_enum {

    use sysinfo::{ProcessExt, System, SystemExt};
    
    // Get processes running on the machine
    pub fn get_processes()
    {
        let sys = System::new_all();
        println!("\n • RUNNING PROCESSES \n");
        for (pid, process) in sys.processes() 
        {
            println!("      [{}] - {} - {} (Path : {})", pid, 
                                                        process.status(), 
                                                        process.name(), 
                                                        process.exe()
                                                        .display());
        }
    }
}