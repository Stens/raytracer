use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::ray_vector::Vec3;
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub struct HittableList<T> {
    pub hittable:  Vec<T>,
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3,) -> HitRecord {
        HitRecord { t, p, normal, front_face:false }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = -1.0*outward_normal.clone();

        } else {
            self.normal = outward_normal.clone();
        }
    }
}

impl<T> HittableList<T> {
    pub fn new(hittablele:Vec<T>) -> HittableList<T> {
        HittableList {
            hittable:hittablele,
        }
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let  temp_rec = &mut HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };
        let mut hit_anything: bool = false;
        let  mut closest_so_far = t_max;

        for object in &self.hittable {
            if object.intersect(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // Use borrowing and life time stuff ?
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        return hit_anything;
    }
}
