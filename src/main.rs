use rascii_art::{charsets::BLOCK, render, RenderOptions};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // RASCII settings
    const WIDTH: u32 = 852;
    let option = RenderOptions::new()
        .width(WIDTH / 3 * 2)
        .colored(true)
        .charset(BLOCK);

    // Number of images
    // const COUNT: usize = 8198;
    const COUNT: usize = 100;

    // Render images to texts and write them to files, in parallel
    let start = Instant::now();
    (1..=COUNT).into_par_iter().for_each(|i| {
        let txt_path = format!("tmpoutput/pmr-{i:0>4}.txt");
        let mut txt = File::create(&txt_path).expect("Creating this file should be fine.");

        let image_path = format!("frames/pmr-{i:0>4}.png");
        render(&image_path, &mut txt, &option).expect("Rendering should not fail.");
        println!("File {txt_path} rendered.");
    });
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}
