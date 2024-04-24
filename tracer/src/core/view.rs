#[derive(Copy, Clone, Debug, PartialEq)]
pub struct View {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u8,
}

impl View {
    pub fn new(width: u32, height: u32, samples_per_pixel: u8) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
        }
    }
}
