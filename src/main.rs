mod maths;
use maths::*;

mod netpbm;
use netpbm::*;

use std::time::Instant;

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_between(-1.0, 1.0), random_between(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - v.dot(n) * n * 2.0
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -f64::sqrt(1.0 - r_out_parallel.length_squared()) * n;
    r_out_parallel + r_out_perp
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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

    fn at(self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}

// #[derive(Copy, Clone)]
// struct SimpleCamera {
//     origin: Vec3,
//     lower_left: Vec3,
//     vertical: Vec3,
//     horizontal: Vec3,
// }

// impl SimpleCamera {
//     fn new(origin: Vec3, lower_left: Vec3, vertical: Vec3, horizontal: Vec3) -> Self {
//         SimpleCamera {
//             origin,
//             lower_left,
//             vertical,
//             horizontal,
//         }
//     }

//     fn get(self, u: f64, v: f64) -> Vec3 {
//         self.lower_left + self.horizontal * u + self.vertical * v
//     }

//     fn get_ray(self, u: f64, v: f64) -> Ray {
//         Ray::new(self.origin, self.get(u, v) - self.origin)
//     }
// }

#[derive(Copy, Clone)]
struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vertical_fov_degrees: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let origin = lookfrom;
        let lens_radius = aperture / 2.0;

        let theta = deg_to_rad(vertical_fov_degrees);
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit();
        let u = (vup.cross(w)).unit();

        let v = w.cross(u);

        let lower_left =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;

        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left,
            vertical,
            horizontal,
            u,
            v,
            w,
            lens_radius,
        }
    }

    fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

fn ray_color(ray: &Ray, objects: &[Box<dyn Hitable>], depth: i32) -> Vec3 {
    let t_min = 0.0001;
    let t_max = std::f64::INFINITY;

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut closest: HitRecord = HitRecord::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        std::f64::INFINITY,
        false,
        MaterialType::Lambertian {
            albedo: Vec3::new(1.0, 0.0, 1.0),
        },
    );

    for object in objects {
        match object.hit(&ray, t_min, t_max) {
            Some(record) => {
                let close = closest;
                if record.t < close.t {
                    closest = record;
                }
            }
            None => continue,
        }
    }

    let hit_info = closest;

    if hit_info.t < std::f64::INFINITY {
        let scatter_res = hit_info.material.scatter(ray, &hit_info);

        match scatter_res {
            Some((attenuation, scattered)) => {
                return attenuation * ray_color(&scattered, objects, depth - 1)
            }
            None => return Vec3::new(0.0, 0.0, 0.0),
        }
    }

    let unit_vec = ray.dir.unit();
    let t = 0.5 * (unit_vec.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

struct Sphere {
    position: Vec3,
    radius: f64,

    material: MaterialType,
}

impl Sphere {
    fn new(position: Vec3, radius: f64, material: MaterialType) -> Self {
        Sphere {
            position,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let root = f64::sqrt(discriminant);
            let t1 = (-b - root) / (2.0 * a);
            let t2 = (-b + root) / (2.0 * a);

            let t = if t1 < t_max && t1 > t_min {
                t1
            } else if t2 < t_max && t2 > t_min {
                t2
            } else {
                return None;
            };

            let normal = ray.at(t) - self.position;

            Some(HitRecord::new(
                ray.at(t),
                normal,
                t,
                ray.dir.dot(normal) < 0.0,
                self.material,
            ))
        }
    }
}

#[derive(Clone, Copy)]
enum MaterialType {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f64 },
    Dialectric { refractive_index: f64 },
}

#[derive(Clone, Copy)]
struct HitRecord {
    position: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: MaterialType,
}

impl HitRecord {
    fn new(position: Vec3, normal: Vec3, t: f64, front_face: bool, material: MaterialType) -> Self {
        let mut normal = if front_face { normal } else { -normal };
        normal = normal.unit();
        HitRecord {
            position,
            normal,
            t,
            front_face,
            material,
        }
    }
}
trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match &self {
            MaterialType::Lambertian { albedo } => {
                let scatter_direction = rec.normal + random_in_hemisphere(rec.normal);
                let scattered = Ray::new(rec.position, scatter_direction);
                let attenuation = *albedo;
                Some((attenuation, scattered))
            }
            MaterialType::Metal { albedo, fuzziness } => {
                let reflected = reflect(ray.dir.unit(), rec.normal);
                let scattered = Ray::new(
                    rec.position,
                    reflected + *fuzziness * (random_in_hemisphere(rec.normal)),
                );
                let attenuation = albedo;
                if scattered.dir.dot(rec.normal) > 0.0 {
                    Some((*attenuation, scattered))
                } else {
                    None
                }
            }
            MaterialType::Dialectric { refractive_index } => {
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                let etai_over_etat = if rec.front_face {
                    1.0 / refractive_index
                } else {
                    *refractive_index
                };

                let unit_direction = ray.dir.unit();
                let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                if etai_over_etat * sin_theta > 1.0 {
                    let reflected = reflect(unit_direction, rec.normal);
                    let scattered = Ray::new(rec.position, reflected);
                    return Some((attenuation, scattered));
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if random_01() < reflect_prob {
                    let reflected = reflect(unit_direction, rec.normal);
                    let scattered = Ray::new(rec.position, reflected);
                    return Some((attenuation, scattered));
                }

                let refracted = refract(unit_direction, rec.normal, etai_over_etat);
                let scattered = Ray::new(rec.position, refracted);
                Some((attenuation, scattered))
            }
        }
    }
}

fn make_random_scene() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        MaterialType::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_01();

            let center = Vec3::new(
                a as f64 + 0.9 * random_01(),
                0.2,
                b as f64 + 0.9 * random_01(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(random_01(), random_01(), random_01());
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialType::Lambertian { albedo },
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::new(random_between(0.5, 1.0), random_between(0.5, 1.0), 1.0);
                    let fuzziness = random_between(0.0, 0.5);
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialType::Metal { albedo, fuzziness },
                    )));
                } else {
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialType::Dialectric {
                            refractive_index: 1.5,
                        },
                    )));
                }
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        MaterialType::Dialectric {
            refractive_index: 1.5,
        },
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        MaterialType::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        MaterialType::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        },
    )));

    objects
}

fn main() {
    println!("Hello, raytracer!");

    let image_width = 1920;
    let image_height = 1080;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let aspect_ratio = image_width as f64 / image_height as f64;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut pixels: Vec<f64> = vec![];

    let objects = make_random_scene();

    // let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     MaterialType::Lambertian {
    //         albedo: Vec3::new(0.7, 0.3, 0.3),
    //     },
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     MaterialType::Lambertian {
    //         albedo: Vec3::new(0.8, 0.8, 0.0),
    //     },
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     MaterialType::Metal {
    //         albedo: Vec3::new(0.8, 0.6, 0.2),
    //         fuzziness: 1.0,
    //     },
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     MaterialType::Dialectric {
    //         refractive_index: 1.5,
    //     },
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     MaterialType::Dialectric {
    //         refractive_index: 1.5,
    //     },
    // )));

    println!("Start rendering");
    let start_time = Instant::now();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = ((i as f64) + random_01()) / image_width as f64;
                let v: f64 = (((image_height - 1 - j) as f64) + random_01()) / image_height as f64;

                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &objects, max_depth);
            }

            color = color / (samples_per_pixel as f64);

            pixels.push(color.x);
            pixels.push(color.y);
            pixels.push(color.z);
        }
    }

    println!("Done! ({:?})", start_time.elapsed());

    println!("Generating image!");

    let output_pixels: Vec<u8> = pixels
        .iter()
        // Do gamma correction
        .map(|&x| f64::sqrt(x))
        // Clamp values between 0 and 1
        .map(|x| clamp(x, 0.0, 0.9999))
        // Convert to 0 -> 256 range
        .map(|x| (255.9 * x))
        .map(|x| x as u8)
        .collect();

    let _res = create_ppm("result.ppm", &output_pixels, image_width, image_height);
}
