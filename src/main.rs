mod hit_record;
mod camera;
mod sphere;
mod ray_vector;
mod ray;
mod material;

use rand::Rng;
use std::sync::Arc;
use std::f32::INFINITY;
use crossbeam::channel::unbounded;
use num_cpus;


use indicatif:: ProgressIterator;
use image::{Rgb, RgbImage, imageops};

use camera::Camera;
use ray_vector::Vec3;
use ray::Ray;
use hit_record::{HitRecord, Hittable, HittableList};
use sphere::Sphere;
use material::Dielectric;



fn ray_color<T: Hittable>(ray: &Ray, world: &HittableList<T>, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let rec =  &mut HitRecord::new(
        0.0,
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(0.0,0.0,0.0), 
        Arc::new(material::Lambertian::new(Vec3::new(0.0,0.0,0.0)))
    );
    if world.intersect(ray,0.001,INFINITY,rec) {
        if let Some((albedo, scattered)) = rec.material.scatter(ray, rec) {
            return albedo * ray_color(&scattered, world, depth-1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().make_unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);

    return (1.0-t)*Vec3::new(1.0,1.0,1.0)+t*Vec3::new(0.5,0.7,1.0);
}


fn render_pixels<T: Hittable>(
                            x_range: (usize, usize), 
                            y_range: (usize, usize), 
                            camera: Camera, 
                            world: HittableList<T>, 
                            samples_per_pixel: u32,
                            img_w:u32,
                            img_h: u32,
                            max_depth:u32) ->Vec<(usize, usize, Rgb<u8>)> {
    let mut pixels: Vec<(usize, usize, Rgb<u8>)> = Vec::new();
    let mut rng = rand::thread_rng();
    for y in (y_range.0..y_range.1).progress() {
        for x in x_range.0..x_range.1 {
            let mut pixel_color = Vec3::new(0.0,0.0,0.0);
            for _ in 0..samples_per_pixel {
                let random_u: f32 = rng.gen();
                let random_v: f32 = rng.gen();

                let v = ((y as f32) + random_v) / ((img_h - 1) as f32);
                let u = ((x as f32) + random_u) / ((img_w - 1) as f32);


                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            let pixel = Rgb(pixel_color.clamp_color(samples_per_pixel).to_color_vec());
            pixels.push((x, y, pixel));
        }
    }
    return pixels;
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 512;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 20;
    let img = &mut  RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    
    // Camera
    const VERTICAL_FOV: f32 = 90.0;
    let look_from: Vec3 = Vec3::new(1.0,1.0,1.0);
    let look_at: Vec3 = Vec3::new(0.0,0.0,-1.0);
    let view_up: Vec3 = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at, 
        view_up, 
        VERTICAL_FOV, 
        ASPECT_RATIO, 
        aperture, 
        dist_to_focus
    );    
    
    // World
    let mut hit_list= Vec::new();

    let mat_ground = Arc::new(material::Lambertian::new(Vec3::new(0.8,0.8,0.0)));
    let mat_right = Arc::new(material::Metal::new(Vec3::new(0.8,0.6,0.2),0.3));
    let mat_center = Arc::new(material::Lambertian::new(Vec3::new(0.1,0.2,0.5)));
    let mat_left = Arc::new(material::Dielectric::new(1.5));
    let mat_left_inner = Arc::new(Dielectric::new(1.5));

    hit_list.push(Sphere::new(Vec3::new(0.0,-100.5, -1.0), 100.0, mat_ground));
    hit_list.push(Sphere::new(Vec3::new(0.0,0.0, -1.0), 0.5, mat_center));
    hit_list.push(Sphere::new(Vec3::new(-1.0,0.0, -1.0), 0.5, mat_left));
    hit_list.push(Sphere::new(Vec3::new(1.0,0.0, -1.0), 0.5, mat_right));
    hit_list.push(Sphere::new(Vec3::new(-1.0,0.0, -1.0), -0.4, mat_left_inner));

    let world  = HittableList::<Sphere>::new(hit_list,);

    // Multiprocessing
    let num_threads = num_cpus::get();

    // let split_x = (IMAGE_WIDTH / num_threads as u32) as usize;
    let split_y = (IMAGE_HEIGHT / num_threads as u32) as usize;
    let (tx, rx) = unbounded();
    crossbeam::scope(|scope| {
        for i in 0..num_threads {
            let x_range = (0 as usize, IMAGE_WIDTH as usize);
            let y_range = (i*split_y, (i+1)*split_y);
            let camera = cam.clone();
            let world = world.clone();
            let tx = tx.clone();
            // Spawn thread
            scope.spawn(move |_| {
                let pixels = render_pixels(x_range, y_range, camera, world, SAMPLES_PER_PIXEL, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_DEPTH);
                tx.send(pixels).unwrap();
            });
        }
        drop(tx);

        for pixels in rx {
            for (x, y, pixel) in pixels {
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }).unwrap();

    imageops::rotate180_in_place(img);
    imageops::flip_horizontal_in_place(img);
    img.save("t.png").expect("Failed to save image");
}
