extern crate minifb;
extern crate shade_tree;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SAMPLES: usize = 1; // Increase for better anti-aliasing

fn window_run() {
    let buffer = shade_tree::render(WIDTH, HEIGHT, SAMPLES);

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

    window_run();
}
