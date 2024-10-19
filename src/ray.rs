use super::vec3::Vec3;

use Vec3 as Point3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(new_o: Point3, new_d: Vec3) -> Ray {
        Ray {
            orig: new_o,
            dir: new_d,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
