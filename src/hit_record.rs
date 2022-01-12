use std::sync::Arc;


// use crate::material::Lambertian;
use crate::ray::Ray;
use crate::ray_vector::Vec3;
use crate::material::{Scatter, Lambertian};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatter>,
    pub front_face: bool,
}
#[derive(Clone)]
pub struct HittableList<T> {
    pub hittable:  Vec<T>,
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, mat: Arc<dyn Scatter> ) -> HitRecord {
        HitRecord { t, p, normal, material: mat, front_face:false }
    }

    pub fn front_face_set_normal(&mut self, ray: &Ray) -> () {
         self.front_face =  ray.direction().dot(&self.normal) < 0.0;
        if !self.front_face {
            self.normal = -1.0 * self.normal;
        }
    }
}

impl<T> HittableList<T> {
    pub fn new(hittable:Vec<T>) -> HittableList<T> {
        HittableList {
            hittable:hittable,
        }
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let  temp_rec = &mut HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
            front_face: false,
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
                rec.material = temp_rec.material.clone();
                rec.front_face = temp_rec.front_face;
            }
        }
        return hit_anything;
    }
}
