mod client; // includes a module
mod image;
mod complex;
mod mandelbrot;
mod test;

fn main() {

    let (width, height, max_iterations) = match client::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    println!("Generating Mandelbrot for {}x{} image (max_iterations: {})", width, height, max_iterations);

    let image = mandelbrot::generate_image(width, height, max_iterations);

    let mandelbrot_pixel_count = image.get_mandelbrot_pixels();

    println!("Pixels in the set: {}", mandelbrot_pixel_count);

    client::save_to_file(&image, "mandelbrot.ppm");
}
