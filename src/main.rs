use rascii_art::{render, RenderOptions};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // RASCII settings
    let charset = &[" ", "░", "▒", "▓", "█"];
    let option = RenderOptions::new()
        .width(568)
        .colored(true)
        .charset(charset);

    // Number of images
    // const COUNT: usize = 8198;
    const COUNT: usize = 8;

    // Render images to texts and write them to files, in parallel
    let start = Instant::now();
    (1..=COUNT).into_par_iter().for_each(|i| {
        let image_path = format!("frames/pmr-{i:0>4}.png");
        let txt_path = format!("tmpoutput/pmr-{i:0>4}.txt");

        let start = Instant::now();
        let mut txt = File::create(&txt_path).expect("");
        let duration = start.elapsed();
        println!("Time elapsed for creating a file is: {:?}", duration);

        let start = Instant::now();
        render(&image_path, &mut txt, &option).expect("");
        let duration = start.elapsed();
        println!("Time elapsed for rendering an image is: {:?}", duration);
    });
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    // let mut stdout = stdout();
    // for  in  {
    //     stdout.execute(cursor::MoveTo(0, 1))?;
    // }

    Ok(())
}
