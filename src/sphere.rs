use super::HitRecord;
use super::Hittable;
use crate::ray::Ray;
use crate::ray_vector::Vec3;

pub struct Sphere {
    radius: f32,
    center: Vec3,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32)-> Sphere {
        Sphere {
            radius: r,
            center: cen,
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool {
        let r_dir = r.direction();
        let oc = r.origin() - self.center;
        let a = r_dir.dot(&r_dir);
        let b = oc.dot(&r_dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (discriminant).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + (discriminant).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
