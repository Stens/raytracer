use crate::{ray_vector::Vec3, ray::Ray};

#[derive(Debug, Copy, Clone)]pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from:Vec3, 
        look_at: Vec3, 
        view_up:Vec3, 
        vertical_fov: f32, 
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {

        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let viewport_height: f32 = 2.0 * (theta/2.0).tan();
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let cw = (look_from - look_at).make_unit_vector();
        let cu = view_up.cross(&cw).make_unit_vector();
        let cv = cw.cross(&cu);


        let h =  viewport_width*cu*focus_dist;
        let v =  viewport_height*cv*focus_dist;
        Camera {
            origin: look_from,
            horizontal: h,
            vertical: v,
            lower_left_corner: look_from - h/2.0 - v/2.0 - cw* focus_dist,
            cu: cu,
            cv: cv,
            lens_radius: aperture/2.0,
        }
    }

    pub fn get_ray(self, u:f32, v:f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu*rd.x() + self.cv*rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}