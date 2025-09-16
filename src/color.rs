use super::vec3::Vec3 as Color;

pub fn write_color(pixel_color: Color) -> String {
    let rbyte = (255.999 * pixel_color.x()) as u8;
    let gbyte = (255.999 * pixel_color.y()) as u8;
    let bbyte = (255.999 * pixel_color.z()) as u8;

    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}
