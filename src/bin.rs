extern crate minifb;
extern crate shade_tree;

use minifb::{Key, Window, WindowOptions};
use std::env;

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

    window_run(samples);
}
