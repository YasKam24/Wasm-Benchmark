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

    // CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <angle_in_radians>", args[0]);
        return;
    }

    // Parse angle
    let angle: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            return;
        }
    };

    // Compute cosine
    let result = angle.cos();
    println!("Cosine of {} radians is: {}", angle, result);

    // Compute execution duration
    let duration = start.elapsed();
    let duration_s = duration.as_secs_f64();

    println!("Execution time: {:.9} seconds", duration_s);
}
