use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn main() {
    // Record start time
    let start_sys = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_ns = start_sys.as_nanos();

    println!("Start time: {} ns", start_ns);

    let start = Instant::now(); // high-resolution timer

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <number>", args[0]);
        return;
    }

    // Parse the number
    let n: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a valid integer.");
            return;
        }
    };

    // Print factors
    let mut count = 0;
    for i in 1..=n {
        if n % i == 0 {
            println!("factor: {}", i);
            count += 1;
        }
    }
    println!("Total factors: {}", count);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
