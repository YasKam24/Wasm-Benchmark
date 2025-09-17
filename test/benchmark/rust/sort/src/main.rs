use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
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

    // Simple pseudo-random array generation without external dependencies
    let mut arr: Vec<i32> = Vec::with_capacity(size);
    for i in 0..size {
        arr.push(((i * 17 + 13) % 1000) as i32); // Simple pseudo-random pattern
    }
    bubble_sort(&mut arr);

    println!("Sorted array of size {}", size);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
