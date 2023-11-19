use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use math::Vec3;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use crate::core::spectrum::Spectrum;

#[derive(Clone)]
pub struct Film {
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    pub pixels: Vec<Spectrum>
}

impl Film {
    pub fn new(width: u32, height: u32, file_name: &str) -> Self {
        Self {
            width,
            height,
            file_name: String::from(file_name),
            pixels: vec![Spectrum::ColorRGB(Vec3::from(0.)); width as usize * height as usize]
        }
    }

    pub fn set_pixel(&mut self, x:u32, y:u32, color: Spectrum) {
        let index = (x + self.width * y) as usize;
        self.pixels[index] = color;
    }

    pub fn set_pixels(&mut self, pixels: Vec<Spectrum>) {
        self.pixels = pixels;
    }

    pub fn write_image(&self) {
        let now: DateTime<Utc> = Utc::now();
        log::info!("UTC now is: {}", now);
        let path_ppm_string = format!("output/{}-{}.ppm", self.file_name,now.format("%v-%H-%M-%S"));
        let path_ppm = Path::new(&path_ppm_string);

        let mut img_png: RgbImage = ImageBuffer::new(self.width, self.height);
        let mut file = match File::create(&path_ppm) {
            Ok(file) => file,
            Err(why) => panic!("couldn't create {}: {}", path_ppm.display(), why),
        };

        let mut image: String = format!("P3\n{} {}\n255\n", self.width, self.height);
        for y in (0..=self.height-1).rev() {
            for x in 0..self.width {
                let index = (x + self.width * y) as usize;
                let Spectrum::ColorRGB(color) = &self.pixels[index];
                let ir = (255.99*color.r()) as u8;
                let ig = (255.99*color.g()) as u8;
                let ib = (255.99*color.b()) as u8;
                image.push_str(&format!("{} {} {}\n", ir, ig, ib));

                img_png.put_pixel(x, y, image::Rgb([ir, ig, ib]));
            }
        }

        match file.write_all(image.as_bytes()) {
            Ok(_) => log::info!("successfully wrote image to {}", path_ppm.display()),
            Err(why) => panic!("couldn't write image to {}: {}", path_ppm.display(), why),
        };

        let path_png_string = format!("output/{}-{}.png", self.file_name,now.format("%v-%H-%M-%S"));
        let path_png = Path::new(&path_png_string);
        match img_png.save(path_png) {
            Ok(_) => log::info!("successfully wrote image to {}", path_png.display()),
            Err(why) => panic!("couldn't write image to {}: {}", path_png.display(), why),
        };


    }
}

