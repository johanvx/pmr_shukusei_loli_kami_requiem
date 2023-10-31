use crossterm::{cursor, ExecutableCommand};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use tokio::sync::mpsc;
use tokio::time::{sleep_until, Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Frame settings
    let frame_rate = 32;
    let frame_height = 85;

    // Channel settings
    let buffer_size = frame_height * 2;
    let (sender, receiver) = mpsc::channel(buffer_size);

    // The rendered video
    let file_path = "output/pmr.txt";

    // Task for reading frames
    let reader_task = tokio::spawn(async move {
        if let Err(err) = read_frames(sender, frame_height, &file_path).await {
            eprintln!("Error reading frames: {}", err);
        }
    });

    // Task for writing frames
    let writer_task = tokio::spawn(async move {
        if let Err(err) = write_frames(receiver, frame_rate).await {
            eprintln!("Error writing frames: {}", err);
        }
    });

    // Wait for both tasks to finish
    let (_, _) = tokio::join!(reader_task, writer_task);

    Ok(())
}

async fn read_frames(
    sender: mpsc::Sender<Vec<String>>,
    frame_height: usize,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let lines = read_lines(file_path)?;
    let mut frame_lines = Vec::with_capacity(frame_height);

    for line in lines {
        frame_lines.push(line?);

        if frame_lines.len() == frame_height {
            // Send the complete frame
            sender.send(frame_lines.clone()).await?;
            frame_lines.clear();
        }
    }

    Ok(())
}

async fn write_frames(
    mut receiver: mpsc::Receiver<Vec<String>>,
    frame_rate: u32,
) -> Result<(), io::Error> {
    let frame_duration = Duration::from_secs(1) / frame_rate;

    let mut stdout = io::stdout();
    let start = Instant::now();

    while let Some(frame) = receiver.recv().await {
        let frame_start = Instant::now();

        // Move to the first line
        stdout.execute(cursor::MoveTo(0, 1))?;

        // Print the complete frame
        for line in &frame {
            println!("{}", line);
        }

        // Panic if elapsed time is longer than frame_duration
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed > frame_duration {
            panic!("Cannot play in {} fps", frame_rate);
        }

        // Sleep if elapsed time is shorter than frame_duration
        let frame_end = frame_start + frame_duration;
        if frame_end > Instant::now() {
            sleep_until(frame_end).await;
        }
    }

    let duration = start.elapsed();
    println!("Elapsed {:?}", duration);

    Ok(())
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
