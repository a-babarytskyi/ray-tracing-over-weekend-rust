mod color;
mod vec3;

use vec3::Vec3;

fn main() {
    let image_heigt = 256;
    let image_width = 256;

    print!("P3\n{} {}\n255\n", image_width, image_heigt);

    for i in 0..image_heigt {
        eprint!("Scanlines remaining: {}\r", image_heigt - i - 1);
        for j in 0..image_width {
            let color = Vec3::from_values(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_heigt - 1) as f64,
                (i + j) as f64 / 2.0 / (image_width - 1) as f64,
            );

            color::write_color(color);
        }
    }
    eprintln!("\nDone.");
}
