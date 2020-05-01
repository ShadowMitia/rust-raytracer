use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn add(self, vec: Self) -> Self {
        Vec3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }

    fn sub(self, vec: Self) -> Self {
        Vec3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }

    fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn mult(self, vec: Vec3) -> Self {
        Vec3 {
            x: self.x * vec.x,
            y: self.y * vec.y,
            z: self.z * vec.z,
        }
    }

    fn div(self, t: f32) -> Self {
        self.mult(self.at(1.0 / t))
    }

    fn at(self, t: f32) -> Self {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    fn dot(self, vec:Vec3) -> f32 {
        self.x * vec.x  + self.y * vec.y + self.z * vec.z
    }

    fn cross(self, vec:Vec3) -> Self {
        Vec3{
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }

    fn unit(self) -> Self {
        self.div(self.length())
    }
}

#[cfg(tests)]
mod vec3_tests {
    use super::*;
}

fn create_ppm(name: &str, pixels: &Vec<u8>, width: u32, height: u32) -> std::io::Result<()> {
    let header = format!("{}\n{} {}\n{}\n", "P6", width, height, 255);

    let mut file = File::create(name)?;
    file.write_all(header.as_bytes())?;
    file.write_all(pixels)?;

    Ok(())
}

fn main() {
    println!("Hello, raytracer!");

    let image_width = 200;
    let image_height = 200;

    let mut pixels: Vec<f32> = vec![];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            pixels.push(i as f32 / image_width as f32);
            pixels.push(j as f32 / image_height as f32);
            pixels.push(0.2);
        }
    }

    let output_pixels = pixels.iter().map(|x| (256f32 * x) as u8).collect();

    println!("Generating image!");
    let _res = create_ppm("gradient.ppm", &output_pixels, image_width, image_height);
    println!("Done!")
}
