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

    let look_from = Vec3f::new(-2.0, 2.0, 1.0);
    let look_at = Vec3f::new(0.2, 0.0, -1.0);
    let distance_to_focus = (look_from - look_at).length();
    let aperture = 0.8;
    let vup = Vec3f::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        width as f64 / height as f64,
        aperture,
        distance_to_focus,
    );

    let mut world = World::default();
    let mut sa = Sphere::new(Vec3f::new(0.2, -0.1, -1.0), 0.4, HitRecord::default());
    sa.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.8, 0.3, 0.3),
    });
    let mut sb = Sphere::new(Vec3f::new(-0.6, 0.0, -1.0), 0.3, HitRecord::default());
    sb.set_material(Material::Dielectric { ri: 1.5 });
    let mut sc = Sphere::new(Vec3f::new(1.2, 0.0, -1.0), 0.3, HitRecord::default());
    sc.set_material(Material::Metal {
        albedo: Vec3f::new(0.8, 0.6, 0.4),
        fuzz: 0.1,
    });
    let mut sd = Sphere::new(Vec3f::new(-3.5, 0.2, -3.0), 0.8, HitRecord::default());
    sd.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.1, 0.2, 0.5),
    });
    let mut se = Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0, HitRecord::default());
    se.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.5, 0.8, 0.2),
    });

    world.add(Box::new(sa));
    world.add(Box::new(sb));
    world.add(Box::new(sc));
    world.add(Box::new(sd));
    world.add(Box::new(se));

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
                color = color / samples as f64;
                color.set_r(color.r().sqrt());
                color.set_g(color.g().sqrt());
                color.set_b(color.b().sqrt());
                let color = Vec3i::new_from_f64(color);
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
