use std::fs::File;
use std::io::prelude::*;

mod maths;
use maths::*;

use rand::prelude::*;

fn random_in_unit_sphere() -> Vec3 {
    let a = random_between(0.0, 2.0 * std::f32::consts::PI);
    let z = random_between(-1.0, 1.0);
    let r = f32::sqrt(1.0 - z * z);

    Vec3::new(r * f32::cos(a), r * f32::sin(a), z)
}

fn random_01() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn random_between(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    fn at(self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

fn create_ppm(name: &str, pixels: &Vec<u8>, width: u32, height: u32) -> std::io::Result<()> {
    let header = format!("{}\n{} {}\n{}\n", "P6", width, height, 255);

    let mut file = File::create(name)?;
    file.write_all(header.as_bytes())?;
    file.write_all(pixels)?;

    Ok(())
}

#[derive(Copy, Clone)]
struct SimpleCamera {
    origin: Vec3,
    lower_left: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl SimpleCamera {
    fn new(origin: Vec3, lower_left: Vec3, vertical: Vec3, horizontal: Vec3) -> Self {
        SimpleCamera {
            origin,
            lower_left,
            vertical,
            horizontal,
        }
    }

    fn get(self, u: f32, v: f32) -> Vec3 {
        self.lower_left + self.horizontal * u + self.vertical * v
    }

    fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.get(u, v) - self.origin)
    }
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hitable>>, depth: i32) -> Vec3 {
    let t_min = 0.0;
    let t_max = 0.0;

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut closest: Option<HitRecord> = None;

    for object in objects {
        match object.hit(&ray, t_min, t_max) {
            Some(record) => {
                let close = closest.unwrap_or(HitRecord::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    std::f32::INFINITY,
                    false,
                ));
                if record.t < close.t {
                    closest = Some(record.clone());
                }
            }
            None => continue,
        }
    }

    let color = match closest {
        Some(_) => {
            let hit_info = closest.unwrap();
            let target = hit_info.position + hit_info.normal + random_in_unit_sphere();
            ray_color(
                &Ray::new(hit_info.position, target - hit_info.position),
                &objects,
                depth - 1,
            ) * 0.5
        }
        None => {
            let unit_vec = ray.dir.unit();
            let t = 0.5 * (unit_vec.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    };

    color
}

struct Cube {}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None
    }
}

struct Circle {
    position: Vec3,
    radius: f32,
}

impl Circle {
    fn new(position: Vec3, radius: f32) -> Self {
        Circle { position, radius }
    }
}

impl Hitable for Circle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            // TODO: send both result of quadratic equation?
            let t = (-b - f32::sqrt(discriminant)) / (2.0 * a);

            if t < 0.1 {
                return None;
            }

            let N = ray.at(t) - self.position;

            Some(HitRecord::new(ray.at(t), N, t, ray.dir.dot(N) < 0.0))
        }
    }
}

#[derive(Copy, Clone)]
struct HitRecord {
    position: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new(position: Vec3, normal: Vec3, t: f32, front_face: bool) -> Self {
        let normal = if front_face { normal } else { -normal };

        HitRecord {
            position,
            normal,
            t,
            front_face,
        }
    }
}
trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

fn main() {
    println!("Hello, raytracer!");

    let image_width = 200;
    let image_height = 100;

    let samples_per_pixel = 100;

    let max_depth = 50;

    let camera = SimpleCamera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
    );

    let mut pixels: Vec<f32> = vec![];

    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Circle::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Circle::new(Vec3::new(-1.0, -1.0, -1.0), 0.5)));
    objects.push(Box::new(Circle::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for j in 0..image_height {
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f32 = ((i as f32) + random_01()) / image_width as f32;
                let v: f32 = (((image_height - 1 - j) as f32) + random_01()) / image_height as f32;

                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &objects, max_depth);
            }

            color = color / (samples_per_pixel as f32);

            pixels.push(color.x);
            pixels.push(color.y);
            pixels.push(color.z);
        }
    }

    let output_pixels: Vec<u8> = pixels
        .iter()
        .map(|&x| f32::sqrt(x))
        .map(|x| clamp(x, 0.0, 0.9999))
        .map(|x| (255.9 * x))
        .map(|x| x as u8)
        .collect();

    println!("Generating image!");
    let _res = create_ppm("normal.ppm", &output_pixels, image_width, image_height);
    println!("Done!")
}
