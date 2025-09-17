use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &val) in arr.iter().enumerate() {
        if val == target {
            return Some(i);
        }
    }
    None
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

    let arr: Vec<i32> = (0..size as i32).collect();
    let target = size as i32 / 2;
    let result = linear_search(&arr, target);

    match result {
        Some(index) => println!("Found target {} at index {} in array of size {}", target, index, size),
        None => println!("Target {} not found in array of size {}", target, size),
    }

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
