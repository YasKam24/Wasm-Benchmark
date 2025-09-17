use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn factorial(n: u64) -> f64 {
    let mut result = 1.0;
    for i in 2..=n {
        result *= i as f64;
    }
    result
}

fn main() {
    // Record start time
    let start_sys = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_ns = start_sys.as_nanos();

    println!("Start time: {} ns", start_ns);

    let start = Instant::now(); // high-resolution timer

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <n>", args[0]);
        return;
    }

    let n: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            return;
        }
    };

    let mut sum = 0.0;
    for i in 1..=n {
        sum += factorial(i);
    }

    println!("Sum of factorials from 1 to {} is: {}", n, sum);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
