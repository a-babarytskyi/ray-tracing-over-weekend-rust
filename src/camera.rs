use crate::hittables::{HitRecord, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random_range;
use std::sync::Arc;
use std::thread;
use Vec3 as Color;
use Vec3 as Point3;

pub struct Camera {
    image_width: i32,
    samples_per_pixel: i32,
    image_height: i32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    camera_center: Vec3,      // Camera center
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
}

impl Camera {
    pub fn init(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
        // Calculate the image height, and ensure that it's at least 1.

        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

        let image_height = if image_height < 1 { 1 } else { image_height };

        // Camera

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::zero();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.

        let viewport_u = Vec3::from_values(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from_values(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = camera_center
            - Vec3::from_values(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        Self {
            image_height,
            image_width,
            samples_per_pixel,
            pixel_samples_scale,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(self: Arc<Self>, world: Arc<HittableList>) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        // Calculate the location of the upper left pixel.

        let avail_threads = std::thread::available_parallelism().unwrap();

        eprintln!("Available cores: {avail_threads:?}");

        let chunk_dim = (self.image_height as usize / avail_threads) as i32;

        let chunks_x = 1; // Right now too lazy to implement horizontal chunking If I keep using ppm, I need to concatenate lines from each chunk in proper order to output correct image.

        let chunks_y = (self.image_height + chunk_dim - 1) / chunk_dim;

        eprintln!("X: {chunks_x}, Y: {chunks_y}");
        let n: i32 = chunks_x * chunks_y;

        let mut slots = Vec::with_capacity(n as usize);

        let mut handles = Vec::with_capacity(n as usize);

        for x in 0..chunks_x {
            let start_x = x * chunk_dim;
            let end_x = if x < (chunks_x - 1) {
                (x + 1) * chunk_dim
            } else {
                self.image_width
            };
            for y in 0..chunks_y {
                let start_y = y * chunk_dim;
                let end_y = if y < (chunks_y - 1) {
                    (y + 1) * chunk_dim
                } else {
                    self.image_height
                };
                let index = y * chunks_x + x;
                eprintln!("Chunk index: {}, x: {}, y: {}", index, x, y);
                eprintln!(
                    "Chunk {}: x_start: {}, x_end: {}, y_start: {}, y_end: {}",
                    index, start_x, end_x, start_y, end_y
                );
                let cam = Arc::clone(&self);
                let world_ref = Arc::clone(&world);

                handles.push(thread::spawn(move || {
                    (
                        index,
                        cam.calculate_chunk(index, start_x, end_x, start_y, end_y, &world_ref),
                    )
                }));
            }
        }

        for handle in handles {
            let (index, result) = handle.join().unwrap();
            slots.insert(index as usize, result);
        }

        let final_string: String = slots.into_iter().collect();

        println!("{}", final_string);
    }

    pub fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::from_values(
            random_range(0.0..1.0) - 0.5,
            random_range(0.0..1.0) - 0.5,
            0.,
        );
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    pub fn write_color(mut pixel_color: Color) -> String {
        let res = (pixel_color.clamp(0.0, 0.999) * 255.999).rounded();

        format!("{} {} {}\n", res[0], res[1], res[2])
    }

    pub fn ray_color(r: &Ray, world: &HittableList) -> Color {
        let mut hit_rec = HitRecord::new();
        if world.hit(
            r,
            Interval::new_from_values(f64::INFINITY, 0.0),
            &mut hit_rec,
        ) {
            return 0.5 * (hit_rec.normal + Color::from_values(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::from_values(1.0, 1.0, 1.0)
            + a * Color::from_values(0.5, 0.7, 1.0);
    }

    pub fn calculate_chunk(
        &self,
        chunk_num: i32,
        x_start: i32,
        x_end: i32,
        y_start: i32,
        y_end: i32,
        world: &Arc<HittableList>,
    ) -> String {
        let mut chunk_res = String::new();
        for j in y_start..y_end {
            for i in x_start..x_end {
                let mut pixel_color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += Camera::ray_color(&r, world);
                }

                chunk_res
                    .push_str(Camera::write_color(self.pixel_samples_scale * pixel_color).as_str());
            }
        }
        eprintln!("Populating slot {} with chunk", chunk_num);
        chunk_res
    }
}
