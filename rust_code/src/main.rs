// use add::add;

// fn main() {
//     let x = 1;
//     let y = 500;
//     let z = add(&x, &y);
//     println!("The addition of {} and {} is {}", x, y, z);
// }

use rust_code::read;
use std::time::{Duration, Instant};
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    read()?;
    let elapsed_time = start_time.elapsed() / 1000;
    let system = System::new_all();
    let memory_usage = system.used_memory() / (1024 * 1024);

    println!("Total time taken: {:?}", elapsed_time);
    println!("Total memory used: {} MB", memory_usage);

    Ok(())
}
