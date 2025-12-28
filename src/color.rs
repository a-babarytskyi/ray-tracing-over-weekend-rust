
use super::vec3::Vec3 as Color;


pub fn write_color(pixel_color: Color) -> String {
    
    let res = (pixel_color*255.999).rounded(); 
    format!("{} {} {}\n", res[0], res[1], res[2])
}
