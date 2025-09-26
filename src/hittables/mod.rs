use crate::{ray::Ray, Vec3};

use Vec3 as Point3;

pub trait Hittable:  Send + Sync {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    r: f64
}

impl Sphere {
    pub fn new(center: Vec3, r: f64) -> Sphere {
        Sphere { center, r }
    }
}


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::from_values(0., 0., 0.),
            normal: Vec3::from_values(0., 0., 0.),
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {*outward_normal } else { -*outward_normal};
    }
    
}


impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
        }
    }
    
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord, ) -> bool {
        let oc = self.center - r.origin();
        let a = &r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.r;
        let outward_normal = (rec.p - self.center) / self.r;
        rec.set_face_normal(r, &outward_normal);

        return true;

    }
}


// class hittable_list : public hittable {
//   public:
//     std::vector<shared_ptr<hittable>> objects;

//     hittable_list() {}
//     hittable_list(shared_ptr<hittable> object) { add(object); }

//     void clear() { objects.clear(); }

//     void add(shared_ptr<hittable> object) {
//         objects.push_back(object);
//     }

//     bool hit(const ray& r, double ray_tmin, double ray_tmax, hit_record& rec) const override {
//         hit_record temp_rec;
//         bool hit_anything = false;
//         auto closest_so_far = ray_tmax;

//         for (const auto& object : objects) {
//             if (object->hit(r, ray_tmin, closest_so_far, temp_rec)) {
//                 hit_anything = true;
//                 closest_so_far = temp_rec.t;
//                 rec = temp_rec;
//             }
//         }

//         return hit_anything;
//     }
// };

//Rust implementation
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(
        &self, r: &Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord {
            p: Point3::from_values(0., 0., 0.),
            normal: Vec3::from_values(0., 0., 0.),
            t: 0.,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }



}

