use std::thread;
use std::time::Duration;

mod color;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

use Vec3 as Color;
use Vec3 as Point3;

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let co = *center - ray.origin();
    let a = &ray.direction().length_squared();
    let h = Vec3::dot(&ray.direction(), &co);
    let c = co.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant >= 0.0 {
        return (h - discriminant.sqrt()) / (a);
    } else {
        return -1.0;
    }
}

pub fn ray_color(r: Ray) -> Color {
    let sphere_center = Point3::from_values(0.0, 0.0, -1.0);

    let t = hit_sphere(&sphere_center, 0.5, &r);

    if t >= 0.0 {
        let x = r.at(t) - sphere_center;
        let n = Vec3::unit_vector(&x);
        return 0.5 * Color::from_values(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::from_values(1.0, 1.0, 1.0) + a * Color::from_values(0.5, 0.7, 1.0);
}

pub fn calculate_chank(
    chunk_num: i32,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    pixel00_loc: Vec3,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
) {
    for j in y_start..y_end {
        let mut chunk_res = String::new();
        for i in x_start..x_end {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);

            chunk_res.push_str(color::write_color(pixel_color).as_str());
        }
    }
    println!("Finished chunk {chunk_num}");
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 3456;

    // Calculate the image height, and ensure that it's at least 1.

    let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

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

    // Calculate the location of the upper left pixel.

    let viewport_upper_left = camera_center
        - Vec3::from_values(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let thread_count = 6;

    let chunk_dim = 100;

    let chunks_x = (image_width + chunk_dim - 1) / chunk_dim;
    let chunks_y = (image_height + chunk_dim - 1) / chunk_dim;

    println!("X: {chunks_x}, Y: {chunks_y}");

    for x in 0..chunks_x {
        let start_x = x * chunk_dim;
        let end_x = if x < (chunks_x - 1) {
            (x + 1) * chunk_dim
        } else {
            image_width
        };
        for y in 0..chunks_y {
            let start_y = y * chunk_dim;
            let end_y = if y < (chunks_y - 1) {
                (y + 1) * chunk_dim
            } else {
                image_height
            };
            println!("Chunk [{x}:{y}] -- X {start_x}-{end_x} :Y {start_y}-{end_y}")
        }
    }

    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // eprintln!("\nDone.");
}
