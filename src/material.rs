use crate::{ray::Ray, hit_record::HitRecord, ray_vector::Vec3};
use crate::Rng;
pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

pub struct Metal{
    albedo: Vec3,
    fuzz: f32,
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo: albedo, fuzz: fuzz }
    }
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx: ref_idx }
    }
    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
         let r0 = ((1.0 -ref_idx)/(1.0 + ref_idx)).powi(2);
         return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
     }
}


impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().make_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        return Some((self.albedo, scattered));
    }
}


impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction().dot(&rec.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }
        return None;
    }
}

impl Scatter for Dielectric {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_direction = ray_in.direction().make_unit_vector();
        let cos_theta = (-1.0) * unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();   
        
        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f32>() < Self::reflectance(cos_theta, refraction_ratio);
    
        let direction: Vec3 = if cannot_refract || will_reflect {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        return Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }
}