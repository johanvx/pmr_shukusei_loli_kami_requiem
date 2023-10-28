use rascii_art::{render_to, RenderOptions};
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // RASCII settings
    const WIDTH: u32 = 852;
    const PMR_CHARSET: &[&str] = &["P", "M", "R"];
    let option = RenderOptions::new()
        .width(WIDTH / 71 * 30)
        .colored(true)
        .charset(PMR_CHARSET);

    // Number of images
    const COUNT: usize = 8198;
    // const COUNT: usize = 100;

    // Render images to texts in parallel
    let start = Instant::now();
    let buffers: Vec<_> = (1..=COUNT)
        .into_par_iter()
        .map(|i| {
            let txt_path = format!("output/pmr-{i:0>4}.txt");

            let mut buffer = String::new();
            let image_path = format!("frames/pmr-{i:0>4}.png");
            render_to(&image_path, &mut buffer, &option).expect("Rendering should not fail.");
            println!("{image_path} rendered.");

            (txt_path, buffer)
        })
        .collect();
    let duration = start.elapsed();
    println!("Time elapsed for rendering images is: {:?}", duration);

    // Save texts to files in parallel
    let save_start = Instant::now();
    buffers.into_par_iter().for_each(|(txt_path, buffer)| {
        fs::write(txt_path, &buffer).expect("Write string to file should not fail.");
    });
    let duration = start.elapsed();
    let save_duration = save_start.elapsed();
    println!("Time elapsed for saving texts is: {:?}", save_duration);
    println!("Total time elapsed is: {:?}", duration);

    Ok(())
}
