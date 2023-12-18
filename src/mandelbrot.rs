use crate::complex::Complex;
use crate::image::{Image, Pixel};

pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut iteration = 0;
    let tolerance = 1e-7; // Define a small tolerance level

    while iteration < max_iterations {
        if z.mag() - 4.0 > tolerance { // Use the tolerance in comparison
            return Some(iteration);
        }
        z = z * z + c;
        iteration += 1;
    }

    None
}

pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
    let mut image = Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 / width as f64 - 0.75) * 3.5;
            let cy = (y as f64 / height as f64 - 0.5) * 2.0;
            let c = Complex { re: cx , im: cy };

            match check_pixel(c, max_iterations) {
                Some(..) => {
                    // Color white
                    if let Some(pixel) = image.get_mut(x, y) {
                        *pixel = Pixel { r: 255, g: 255, b: 255 };
                    }
                },
                None => {
                    // Color black
                    if let Some(pixel) = image.get_mut(x, y) {
                        *pixel = Pixel { r: 0, g: 0, b: 0 };
                    }
                }
            }
        }
    }

    image

}
