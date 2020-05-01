use std::fs::File;
use std::io::prelude::*;

mod maths;
use maths::*;

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

fn ray_color(ray: &Ray) -> Vec3 {
    let cercle = Cercle::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    let t =  hit_sphere(&cercle, &ray);
    if t > 0.0 {
        return ((ray.at(t) - cercle.position).unit() + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_vec = ray.dir.unit();
    let t = 0.5 * (unit_vec.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

struct Cercle {
    position: Vec3,
    radius: f32,
}

impl Cercle {
    fn new(position: Vec3, radius: f32) -> Self {
        Cercle { position, radius }
    }
}

fn hit_sphere(cercle: &Cercle, ray: &Ray) -> f32 {
    let oc = ray.origin - cercle.position;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - cercle.radius * cercle.radius;

    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        // TODO: send both result of quadratic equation?
        (-b - f32::sqrt(discriminant)) / (2.0 * a)
    }

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

    for j in 0..image_height {
        for i in 0..image_width {
            let u: f32 = i as f32 / image_width as f32;
            let v: f32 = (image_height - 1 - j) as f32 / image_height as f32;

            let ray = Ray::new(camera.origin, camera.get(u, v));

            let color = ray_color(&ray);

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
    let _res = create_ppm(
        "normal.ppm",
        &output_pixels,
        image_width,
        image_height,
    );
    println!("Done!")
}
