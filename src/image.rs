#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Pixel {
    pub fn new(r: usize, g: usize, b: usize) -> Self {
        Pixel { r, g, b }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![Pixel::new(0, 0, 0); width * height];
        Image { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_mandelbrot_pixels(&self) -> usize {
        self.data.iter().filter(|&pixel| *pixel == Pixel::new(0, 0, 0)).count()
    }
}
