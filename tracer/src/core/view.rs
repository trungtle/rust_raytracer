pub struct View {
    pub width: u32, 
    pub height: u32
}

impl View {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}