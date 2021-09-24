use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::core::Spectrum;
use crate::math::Vec3;

#[derive(Clone)]
pub struct Film {
    width: u32,
    height: u32,
    file_name: String,
    pixels: Vec<Spectrum>
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

    pub fn write_image(&self) {
        let path_string = format!("output/{}.bmp", self.file_name);
        let path = Path::new(&path_string);
        let display = path.display();

        let mut file = match File::create(&path) { 
            Ok(file) => file,
            Err(why) => panic!("couldn't create {}: {}", display, why),
        };
        
        let mut image: String = format!("P3\n{} {}\n255\n", self.width, self.height);
        for y in (0..=self.height-1).rev() {
            for x in 0..self.width {
                let index = (x + self.width * y) as usize;
                let Spectrum::ColorRGB(color) = &self.pixels[index];
                let ir = (255.99*color.r()) as u32;
                let ig = (255.99*color.g()) as u32;
                let ib = (255.99*color.b()) as u32;
                image.push_str(&format!("{} {} {}\n", ir, ig, ib));    
            }
        }

        match file.write_all(image.as_bytes()) {
            Ok(_) => println!("successfully wrote image to {}", display),
            Err(why) => panic!("couldn't write image to {}: {}", display, why),
        };
    }
}

