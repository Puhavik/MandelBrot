use httparse::Request;
// use url::Url;
mod client; // includes a module
mod image;
mod complex;
mod mandelbrot;
mod test;
use std::io::Write;
use std::io::Read;
use std::{
    fs,
    // io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


type Query = Vec<(String, String)>;
fn parse_query(query: &str) -> Query {
    let mut query_pairs = Vec::new();
    for pair in query.split('&') {
        let mut pair = pair.split('=');
        let key = pair.next().unwrap_or("").to_string();
        let value = pair.next().unwrap_or("").to_string();
        query_pairs.push((key, value));
    }
    query_pairs
}



fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    let bytes_read = stream.read(&mut buf).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = Request::new(&mut headers);
    let result = request.parse(&buf[..bytes_read]);
    
    if let Ok(_) = result {
        if let Some(path) = request.path {
            let query = parse_query(path);
            println!("{:?}", query[0].0);

            if query[0].0 == "/?width" {
                go(from_string_to_usize(query[0].1.clone()), from_string_to_usize(query[1].1.clone()), from_string_to_usize(query[2].1.clone()), from_string_to_usize(query[3].1.clone()));
                let contents = fs::read_to_string("mandelbrot.ppm").unwrap();
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

                stream.write(response.as_bytes()).unwrap();
                return;
            }
        }
    }

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();

}

fn from_string_to_usize(s: String) -> usize {
    let mut result = 0;
    for c in s.chars() {
        result = result * 10 + (c as usize - '0' as usize);
    }
    result
}

fn go(width: usize, height: usize, max_iterations: usize, color: usize) {

    // let (width, height, max_iterations, color) = match client::parse_args() {
    //     Ok(args) => args,
    //     Err(e) => {
    //         eprintln!("Error parsing arguments: {}", e);
    //         return;
    //     }
    // };

    println!("Generating Mandelbrot for {}x{} image (max_iterations: {} and color {})", width, height, max_iterations, color);

    let image = mandelbrot::generate_image(width, height, max_iterations, color);

    let mandelbrot_pixel_count = image.get_mandelbrot_pixels();

    println!("Pixels in the set: {}", mandelbrot_pixel_count);

    client::save_to_file(&image, "mandelbrot.ppm");

}

