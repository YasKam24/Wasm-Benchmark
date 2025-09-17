use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn array_sum(arr: &[i32]) -> i64 {
    arr.iter().map(|&x| x as i64).sum()
}

fn main() {
    // Record start time
    let start_sys = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_ns = start_sys.as_nanos();

    println!("Start time: {} ns", start_ns);

    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        return;
    }

    let size: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            return;
        }
    };

    let arr: Vec<i32> = (1..=size as i32).collect();
    let sum = array_sum(&arr);

    println!("Sum of array of size {} is: {}", size, sum);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
