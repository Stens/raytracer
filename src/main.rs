use std::f32::INFINITY;

use image::{Rgb, RgbImage};
mod ray_vector;
use ray_vector::Vec3;
mod ray;
use ray::Ray;
mod hit_record;
use hit_record::{HitRecord, Hittable, HittableList};
mod sphere;
use sphere::Sphere;

fn ray_color(ray: &mut Ray, world: &Hittable) -> Vec3 {
    let rec =  &mut HitRecord::new(0.0,Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
    if world.intersect(ray,0.0,INFINITY,rec) {
        return 0.5*(rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().make_unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);

    return (1.0-t)*Vec3::new(1.0,1.0,1.0)+t*Vec3::new(0.5,0.7,1.0);
    
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width as f32 / aspect_ratio;
    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut u: f32;
    let mut v: f32;

    let mut x: f32;
    let mut y: f32 = (image_height - 1.0) as f32;
    let mut ray: Ray;
    let mut color: Vec3;

    let mut hit_list= Vec::new();
    hit_list.push(Sphere::new(Vec3::new(0.0,0.0, -1.0), 0.5));
    hit_list.push(Sphere::new(Vec3::new(0.0,-100.5, -1.0), 100.0));


    let world  = HittableList::<Sphere>::new(hit_list, );
    // Render
    while y != 0.0 {
        x = 0.0;
        while x < image_width {
            u = x / (image_width - 1.0);
            v = y / (image_height - 1.0);

            ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            color = ray_color(&mut ray, &world);

            img.put_pixel(x as u32, y as u32, Rgb(color.to_color_vec()));
            x += 1.0;
        }
        y -= 1.0;
    }
    img.save("firstImage.png").expect("Failed to save image");
}
