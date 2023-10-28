use crossterm::{cursor, ExecutableCommand};
use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("output/pmr.txt")?;
    let bufreader = BufReader::new(file);

    // Print one file
    let mut stdout = stdout();
    let start = Instant::now();
    let mut count = 0;
    for line in bufreader.lines() {
        if count == 159 {
            stdout.execute(cursor::MoveToRow(1))?;
            count = 0;
        }
        println!("{}", line?);
        count += 1;
    }
    let duration = start.elapsed();
    println!("Time elapsed for printing one file is: {:?}", duration);

    Ok(())
}
