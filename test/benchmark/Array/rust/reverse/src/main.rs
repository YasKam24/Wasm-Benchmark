use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn reverse_array(arr: &mut [i32]) {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
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

    let mut arr: Vec<i32> = (0..size as i32).collect();
    reverse_array(&mut arr);

    println!("Reversed array of size {}", size);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
