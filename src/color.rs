
use super::vec3::Vec3 as Color;


pub fn write_color(pixel_color: Color) -> String {
    
    let res = pixel_color*255.999;

    format!("{} {} {}\n", res.x() as u8, res.y() as u8, res.z() as u8)
}
