mod camera;
mod ray;
mod sphere;
mod vec;
mod visible;
mod world;

extern crate rand;

use camera::Camera;
use rand::Rng;
use sphere::Sphere;
use vec::Vec3f;
use vec::Vec3i;
use world::World;

pub fn blank_screen(width: usize, height: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];

    for i in buffer.iter_mut() {
        *i = 0xff_ffff;
    }

    buffer
}

pub fn gradient(width: usize, height: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let height_f64 = height as f64;
    let width_f64 = width as f64;

    for y in 0..height {
        for x in 0..width {
            let color = Vec3f::new(y as f64 / height_f64, x as f64 / width_f64, 0.0);
            let color = Vec3i::new_from_f64(color);
            let color = color.to_hex();

            buffer[(y * width) + x] = color;
        }
    }

    buffer
}

pub fn render(width: usize, height: usize, samples: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut buffer: Vec<u32> = vec![0; width * height];

    let origin = Vec3f::new(0.0, 0.0, 0.0);
    let horizontal = Vec3f::new(4.0, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3f::new(-2.0, -1.0, -1.0);
    let camera = Camera::new(origin, horizontal, vertical, lower_left_corner);

    let mut world = World::default();
    world.add(Box::new(Sphere::new(Vec3f::new(0.0, 0.0, 1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3f::new(0.0, -100.0, 1.0), 100.0)));

    for y in 0..height {
        for x in 0..width {
            let mut color = Vec3f::new(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let u = (x as f64 + rng.gen::<f64>()) / width as f64;
                let v = (y as f64 + rng.gen::<f64>()) / height as f64;

                let p = camera::color(camera.get_ray(u, v), &world);
                color = color + p;
            }
            let color = Vec3i::new_from_f64(color / samples as f64);
            let color = color.to_hex();

            // y coordinate starts in the bottom, but in the buffer it starts in the top
            let by = (height - y) - 1;
            buffer[(by * width) + x] = color;
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
