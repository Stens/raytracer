use image::{Rgb, RgbImage};
mod ray_vector;
use crate::ray_vector::Vec3;
mod ray;
use crate::ray::Ray;

fn ray_color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let N = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).make_unit_vector();
        return 0.5 * Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_direction: Vec3 = ray.direction().make_unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    let vecern = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    return vecern;
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
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
            color = ray_color(&ray);

            img.put_pixel(x as u32, y as u32, Rgb(color.to_color_vec()));
            x += 1.0;
        }
        y -= 1.0;
    }
    img.save("firstImage.png").expect("Failed to save image");
}
