use std::u8;

use image::ImageError;
use image::{ExtendedColorType, ImageReader, save_buffer};

use crate::utils::misc_util::is_rectangular;

#[derive(Clone)]
/// A rgb8 image
///
/// * `data`: underlying data for the image, in flattened u8
/// * `dim`: image dimension
pub struct Image {
    data: Vec<u8>,
    dim: (usize, usize),
}

impl Image {
    /// save the image with given name
    ///
    /// * `name`: img name
    pub fn save(&self, name: &str) -> Result<(), ImageError> {
        save_buffer(
            name,
            &self.data,
            self.dim.0 as u32,
            self.dim.1 as u32,
            ExtendedColorType::Rgb8,
        )?;
        Ok(())
    }
    /// generate a image from a flattened pixel array (rgb foramt)
    ///
    /// * `data`: pixel array in rgb8 format
    /// * `dim`: img size
    pub fn from_flatten(data: &Vec<u8>, dim: &(usize, usize)) -> Self {
        Self {
            data: data.clone(),
            dim: dim.clone(),
        }
    }

    /// load a image from a file
    ///
    /// * `path`: img path
    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?;
        Ok(Self {
            data: img.as_bytes().to_vec(),
            dim: (img.width() as usize, img.height() as usize),
        })
    }

    /// create image from nested data array
    ///
    /// * `data`: nested data array
    pub fn new(data: &Vec<Vec<u8>>) -> Self {
        if !is_rectangular(data) {
            panic!("input image data must be rectangular!")
        }
        Self::from_flatten(
            &data.iter().flatten().copied().collect(),
            &(data.len(), data[0].len() / 3),
        )
    }
    /// push a pixel to the end of the image
    ///
    /// * `px`: pixel tuple
    pub fn push_px(&mut self, px: &(u8, u8, u8)) {
        self.data.push(px.0);
        self.data.push(px.1);
        self.data.push(px.2);
    }

    /// get pixel data of a given position
    ///
    /// * `loc`: position, as a pair
    pub fn pixel_data(&self, loc: &(usize, usize)) -> (u8, u8, u8) {
        let base = loc.0 * self.dim.1 as usize + loc.1;
        (self.data[base], self.data[base + 1], self.data[base + 2])
    }
}
