mod client; // includes a module
mod complex;
mod image;
mod mandelbrot;
mod test;

fn main() {
    // uncomment and implement argument parsing, priting an error message in case of a parsing error
    // call client::parse_args() to get width, height and max_iterations from the input arguments

    // if the above was implemented correctly you can uncomment this line
    // println!("Generating Mandelbrot for {}x{} image (max_iterations: {})", width, height, max_iterations);

    // call mandelbrot::generate_image(width, height, max_iterations) and save the result to an image

    // call the get_mandelbrot_pixels() method on the image struct and save the result in mandelbrot_pixel_count

    // if the above line worked you should be able to uncomment this line
    // println!("Pixels in the set: {}", mandelbrot_pixel_count);

    // uncomment and call after you implement the mandelbrot functions, and handle the possible error
    // client::save_to_file(&image, "mandelbrot.ppm");

}
