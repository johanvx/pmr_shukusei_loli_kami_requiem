use rascii_art::{render_to, RenderOptions};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // RASCII settings
    const WIDTH: u32 = 1280;
    const PMR_CHARSET: &[&str] = &["P", "M", "R"];
    let option = RenderOptions::new()
        .width(WIDTH / 5)
        .colored(true)
        .charset(PMR_CHARSET);

    let mut buffer = String::new();
    let image_path = format!("thumbnail.webp");
    render_to(&image_path, &mut buffer, &option).expect("Rendering should not fail.");
    println!("{buffer}");

    Ok(())
}
