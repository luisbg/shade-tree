mod camera;
mod material;
mod ray;
mod sphere;
mod vec;
mod visible;
mod world;

extern crate rand;

use crate::material::Material;
use crate::visible::{HitRecord, Visible};
use camera::Camera;
use rand::Rng;
use rayon::prelude::*;
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
    let mut buffer: Vec<u32> = vec![0; width * height];

    let origin = Vec3f::new(0.0, 0.0, 0.0);
    let horizontal = Vec3f::new(4.0, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3f::new(-2.0, -1.0, -1.0);
    let camera = Camera::new(origin, horizontal, vertical, lower_left_corner);

    let mut world = World::default();
    let mut sa = Sphere::new(Vec3f::new(0.4, 0.0, -1.0), 0.5, HitRecord::default());
    sa.set_material(Material::Metal {
        albedo: Vec3f::new(1.0, 0.2, 0.6),
    });
    let mut sb = Sphere::new(Vec3f::new(-0.6, 0.3, -2.0), 0.3, HitRecord::default());
    sb.set_material(Material::Metal {
        albedo: Vec3f::new(0.2, 1.0, 0.6),
    });
    let mut sc = Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0, HitRecord::default());
    sc.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.6, 0.2, 1.0),
    });

    world.add(Box::new(sa));
    world.add(Box::new(sb));
    world.add(Box::new(sc));

    buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let mut rng = rand::thread_rng();
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut color = Vec3f::default();
                for _s in 0..samples {
                    let u = (x as f64 + rng.gen_range(0.0, 1.0)) / width as f64;
                    let v = ((height - y) as f64 + rng.gen_range(0.0, 1.0)) / height as f64;

                    let p = camera::color(camera.get_ray(u, v), &world, 0);
                    color = color + p;
                }
                let color = Vec3i::new_from_f64(color / samples as f64);
                *pixel = color.to_hex();
            }
        });

    buffer
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
