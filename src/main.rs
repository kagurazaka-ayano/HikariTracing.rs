use std::{env, io::Result};

pub mod objs;
pub mod utils;

use utils::image_util;

use crate::utils::graphic_utils::Noise3D;
use crate::utils::graphic_utils::Perlin;

fn main() -> Result<()> {
    let mut a: Vec<u8> = Vec::new();
    for j in 0..(256 * 256) {
        for i in 252..255 {
            a.push(i);
        }
    }
    let img = image_util::Image::from_flatten(&a, &(256, 256));
    let _ = img.save(&format!("{}/out.png", env::current_dir()?.display()));

    Ok(())
}
