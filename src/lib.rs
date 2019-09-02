mod camera;
mod material;
mod ray;
mod sphere;
mod vec;
mod visible;
mod world;

extern crate indicatif;
extern crate rand;

use crate::material::Material;
use crate::visible::{HitRecord, Visible};
use camera::Camera;
use indicatif::ProgressBar;
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

pub fn generate_random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::default();

    let mut metal = Sphere::new(Vec3f::new(4.0, 1.0, 0.0), 1.0, HitRecord::default());
    metal.set_material(Material::Metal {
        albedo: Vec3f::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    let mut glass = Sphere::new(Vec3f::new(0.0, 1.0, 0.0), 1.0, HitRecord::default());
    glass.set_material(Material::Dielectric { ri: 1.5 });
    let mut lamb = Sphere::new(Vec3f::new(-4.0, 1.0, 0.0), 1.0, HitRecord::default());
    lamb.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.4, 0.2, 0.1),
    });

    world.add(Box::new(glass));
    world.add(Box::new(lamb));
    world.add(Box::new(metal));

    for a in -11..11 {
        for b in -11..11 {
            let mat = rng.gen_range(0.0, 1.0);
            let center = Vec3f::new(
                a as f64 + rng.gen_range(0.0, 0.9),
                0.2,
                b as f64 + rng.gen_range(0.0, 0.9),
            );
            if (center - Vec3f::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut tmp = Sphere::new(center, 0.2, HitRecord::default());
                if mat < 0.8 {
                    // difuse
                    tmp.set_material(Material::Lambertian {
                        albedo: Vec3f::new(
                            rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0),
                        ),
                    });
                } else if mat < 0.95 {
                    // metal
                    tmp.set_material(Material::Metal {
                        albedo: Vec3f::new(
                            0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                            0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                            0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                        ),
                        fuzz: rng.gen_range(0.0, 0.5),
                    });
                } else {
                    // glass
                    tmp.set_material(Material::Dielectric { ri: 1.5 });
                }

                world.add(Box::new(tmp));
            }
        }
    }

    let mut ground = Sphere::new(Vec3f::new(0.0, -1000.0, 0.0), 1000.0, HitRecord::default());
    ground.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(ground));

    world
}

pub fn generate_scene() -> World {
    let mut world = World::default();

    let mut lamb_a = Sphere::new(Vec3f::new(0.3, -0.1, -1.0), 0.4, HitRecord::default());
    lamb_a.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.8, 0.3, 0.3),
    });
    let mut glass = Sphere::new(Vec3f::new(-0.4, 0.0, -1.0), 0.3, HitRecord::default());
    glass.set_material(Material::Dielectric { ri: 1.5 });
    let mut metal = Sphere::new(Vec3f::new(1.2, 0.0, -1.0), 0.3, HitRecord::default());
    metal.set_material(Material::Metal {
        albedo: Vec3f::new(0.8, 0.6, 0.4),
        fuzz: 0.1,
    });
    let mut lamb_b = Sphere::new(Vec3f::new(-3.5, 0.2, -3.0), 0.8, HitRecord::default());
    lamb_b.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.1, 0.2, 0.5),
    });
    let mut ground = Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0, HitRecord::default());
    ground.set_material(Material::Lambertian {
        albedo: Vec3f::new(0.5, 0.8, 0.2),
    });

    world.add(Box::new(lamb_a));
    world.add(Box::new(lamb_b));
    world.add(Box::new(glass));
    world.add(Box::new(metal));
    world.add(Box::new(ground));

    world
}

pub fn render(width: usize, height: usize, samples: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let look_from = Vec3f::new(12.0, 1.0, 3.0);
    let look_at = Vec3f::new(-4.0, 0.0, -1.0);
    let distance_to_focus = 10.0;
    let aperture = 0.4;
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

    let world = generate_random_scene();

    let pb = ProgressBar::new(height as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:70.cyan/blue}]  {percent}%  ({eta})")
            .progress_chars("#-"),
    );

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

            pb.inc(1);
        });
    pb.finish_with_message("Render done");

    buffer
}
