use std::io::{self, Write};

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    
    // Time interval for one frame
    let frame_time = Duration::from_secs_f64(1.0 / 60.0);

    loop {

        // Start the stopwatch
        let frame_start = Instant::now();

        // =========== In Frame ==================
        print!("\x1B[2J\x1B[H"); // Reseting the terminal

        for row in 0..20 {
            for col in 0..10 {
                print!(".");
            }
            println!()
        }

        // flushing the buffer data to terminal with error handling
        if let Err(error) = io::stdout().flush() {
            eprintln!("Could not display the frame: {error}");
            return;
        }
        // =======================================

        // Check the stopwatch
        let elapsed = frame_start.elapsed();

        // If the checked time from the stopwach < time interval for one frame
        if elapsed < frame_time {

            // Pause the program for remaining time.
            thread::sleep(frame_time - elapsed);
        }
    }
}

