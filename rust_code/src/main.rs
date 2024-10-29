
use rust_code::read;
use std::time::Instant;
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = System::new_all();
    system.refresh_memory();
    let start_time = Instant::now();

    system.refresh_memory();
    let initial_memory = system.used_memory();

    read()?;
    let elapsed_time = start_time.elapsed()/1000;

    system.refresh_memory();
    let final_memory = system.used_memory();

    let memory_usage = (final_memory-initial_memory)/(1024*1024);

    println!("Total time taken: {:?}", elapsed_time);
    println!("Total memory used: {} MB", memory_usage);

    Ok(())
}
