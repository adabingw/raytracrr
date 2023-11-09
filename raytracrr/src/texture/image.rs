use image::{io::Reader, RgbImage};
use std::{error::Error, path::Path, sync::Arc};

use crate::{texture::{Texture}, vec::{Vec3, Colour}};

pub struct Image {
    data: RgbImage
}

impl Image {
    pub fn new(filename: impl AsRef<Path>) -> Result<Image, Box<dyn Error + Send + Sync>> {
        let data_file = match Reader::open(filename.as_ref()) {
            Ok(data) => data,
            Err(x) => {
                return Err(format!(
                    "can't open texture image file {}: {}", filename.as_ref().display(), x
                ).into())
            }
        };

        let data = match data_file.decode() {
            Ok(data) => data, 
            Err(x) => {
                return Err(format!(
                    "can't decode texture image file {}: {}", filename.as_ref().display(), x
                ).into())
            }
        };

        let data = data.to_rgb8();
        Ok(Image {data})
    }

    pub fn new_arc(filename: impl AsRef<Path>) -> Arc<Result<Image, Box<dyn Error + Send + Sync>>> {
        Arc::new(Image::new(filename))
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        let w = self.data.width();
        let h = self.data.height();

        // clamp to [0, 1] x [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip v to image coordinates 

        let i = (u * f64::from(w)) as u32;
        let j = (v * f64::from(h)) as u32;

        // clamp integer mapping, since actual coords should < 1.0
        let i = i.min(w - 1);
        let j = j.min(h - 1);

        const COLOUR_SCALE: f64 = 1.0 / 255.0;

        let pixel = self.data.get_pixel(i, j).0;
        let r = pixel[0] as f64;
        let g = pixel[1] as f64;
        let b = pixel[2] as f64;

        COLOUR_SCALE * Colour::new(r, g, b)
    }
}
