extern crate minifb;
extern crate png;
extern crate shade_tree;

use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::BufWriter;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const DEFAULT_SAMPLES: usize = 30; // Increase for better anti-aliasing

fn window_run(samples: usize) {
    let buffer = shade_tree::render(WIDTH, HEIGHT, samples);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}

fn output_file(samples: usize, filename: String) {
    let mut path = env::current_dir().unwrap();
    path.push(filename);
    println!("Rendering to file: {}", path.display());

    let buffer = shade_tree::render(WIDTH, HEIGHT, samples);

    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = shade_tree::vec_from_hex(buffer);
    writer.write_image_data(data.as_slice()).unwrap();
}

fn main() {
    println!("Shade Tree");

    let args: Vec<String> = env::args().collect();
    let samples = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(s) => s,
            Err(e) => {
                println!("Error parsing samples per pixel: {}", e);
                DEFAULT_SAMPLES
            }
        }
    } else {
        DEFAULT_SAMPLES
    };

    if args.len() > 2 {
        output_file(samples, args[2].clone());
    } else {
        window_run(samples);
    };
}
