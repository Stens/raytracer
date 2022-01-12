use std::sync::Arc;

use super::HitRecord;
use super::Hittable;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::ray_vector::Vec3;

#[derive(Clone)]
pub struct Sphere {
    radius: f32,
    center: Vec3,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32, mat: Arc<dyn Scatter>)-> Sphere {
        Sphere {
            radius: r,
            center: cen,
            material: mat,
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
                rec.material = self.material.clone();
                rec.front_face_set_normal(r);
                return true;
            }
            temp = (-b + (discriminant).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                rec.front_face_set_normal(r);
                return true;
            }
        }
        return false;
    }
}
