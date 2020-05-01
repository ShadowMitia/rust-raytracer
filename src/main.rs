use std::fs::File;
use std::io::prelude::*;

mod maths;
use maths::*;

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
#[derive(Copy, Clone)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    fn at(self, t: f32) -> Vec3 {
        self.origin.add(self.dir.mult_float(t))
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
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hitable>>) -> Vec3 {
    let t_min = 0.0;
    let t_max = 0.0;

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

    match closest {
        Some(_) => (closest.unwrap().normal.unit() + Vec3::new(1.0, 1.0, 1.0)) * 0.5,
        None => {
            let unit_vec = ray.dir.unit();
            let t = 0.5 * (unit_vec.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
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
            let u: f32 = i as f32 / image_width as f32;
            let v: f32 = (image_height - 1 - j) as f32 / image_height as f32;

            let ray = Ray::new(camera.origin, camera.get(u, v));

            let color = ray_color(&ray, &objects);

            pixels.push(color.x);
            pixels.push(color.y);
            pixels.push(color.z);

            // color gradient
            // pixels.push(u);
            // pixels.push(v);
            // pixels.push(0.2);
        }
    }

    let output_pixels = pixels.iter().map(|x| (255.9 * x) as u8).collect();

    println!("Generating image!");
    let _res = create_ppm("normal.ppm", &output_pixels, image_width, image_height);
    println!("Done!")
}
