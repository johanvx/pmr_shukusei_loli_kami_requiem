use crossterm::{cursor, ExecutableCommand};
use std::error::Error;
use std::fs::File;
use std::io::{self, stdout, BufRead, BufReader};
use std::path::Path;
use tokio::time::{sleep_until, Instant, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let frame_rate = 32;
    let frame_duration = Duration::from_secs(1) / frame_rate;

    // Set the frame height
    let frame_height= 85;

    let mut stdout = stdout();
    let mut lines = read_lines("output/pmr.txt")?;
    let start = Instant::now();

    while let Some(line) = lines.next() {
        // Start of the frame
        let frame_start = Instant::now();

        // Move to the first line
        stdout.execute(cursor::MoveTo(0, 1))?;

        // Print a frame
        println!("{}", line?);
        for _ in 0..(frame_height - 1) {
            if let Some(line) = lines.next() {
                println!("{}", line?);
            }
        }

        // Sleep if elapsed time is shorter than frame_duration
        let frame_end = frame_start + frame_duration;
        if frame_end > Instant::now() {
            sleep_until(frame_end).await;
        } else {
            panic!("Cannot play in {} fps", frame_rate);
        }
    }

    let duration = start.elapsed();
    println!("Elapsed {:?}.", duration);

    Ok(())
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
