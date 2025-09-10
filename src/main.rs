use std::{env, io::Result};

mod image_util;

fn main() -> Result<()> {
    let mut a: Vec<u8> = Vec::new();
    for j in 0..(256 * 256) {
        for i in 252..255 {
            a.push(i);
        }
    }
    let img = image_util::Image::from_flatten(&a, &(256, 256));
    img.save(&format!("{}/out.png", env::current_dir()?.display()));
    Ok(())
}
