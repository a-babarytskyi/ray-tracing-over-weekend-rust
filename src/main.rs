#![feature(portable_simd)]
#![feature(random)]

mod camera;
mod hittables;
mod interval;
mod ray;
mod vec3;

use std::{sync::Arc, time::Instant};
use vec3::Vec3;

use hittables::{HittableList, Sphere};

use Vec3 as Point3;

use crate::camera::Camera;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 640;
    let samples_per_pixel = 10;

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::from_values(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::new(
        Point3::from_values(0.0, -100.5, -1.0),
        100.0,
    )));

    let start_time = Instant::now();

    let camera = Camera::init(image_width, aspect_ratio, samples_per_pixel);

    Camera::render(Arc::new(camera), Arc::new(world));

    let elapsed_time = start_time.elapsed();
    eprintln!("\nDone. Time taken: {:.2?}", elapsed_time);
}
