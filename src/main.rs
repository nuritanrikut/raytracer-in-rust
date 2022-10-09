mod camera;
mod dielectric;
mod hit_record;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod rng;
mod sphere;
mod utils;
mod vec3;

use camera::*;
use dielectric::*;
use hit_record::*;
use hittable::*;
use hittable_list::*;
use lambertian::*;
use material::*;
use metal::*;
use ray::*;
use rng::*;
use sphere::*;
use std::{env, f64::INFINITY, rc::Rc};
use utils::*;
use vec3::*;

fn simple_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });

    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    }));

    let material1: Rc<dyn Material> = Rc::new(Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1,
    }));

    let material2: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2,
    }));

    let material3: Rc<dyn Material> = Rc::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3,
    }));

    return world;
}

fn random_scene(rng: &mut RandomNumberGenerator) -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });

    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    }));

    let world_center = Point3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };
    let radius = 0.2;

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3 {
                x: a as f64 + 0.9 * rng.random_double(),
                y: 0.2,
                z: b as f64 + 0.9 * rng.random_double(),
            };

            if (center - world_center).length() > 0.9 {
                let material: Rc<dyn Material>;
                let choose_mat = rng.random_double();
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rng.random_vec3();
                    material = Rc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rng.random_vec3_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0, 0.5);
                    material = Rc::new(Metal { albedo, fuzz });
                } else {
                    // glass
                    material = Rc::new(Dielectric { ir: 1.5 });
                }
                world.add(Rc::new(Sphere {
                    center,
                    radius,
                    material,
                }));
            }
        }
    }

    let material1: Rc<dyn Material> = Rc::new(Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1,
    }));

    let material2: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2,
    }));

    let material3: Rc<dyn Material> = Rc::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3,
    }));

    return world;
}

fn ray_color(
    i: i32,
    j: i32,
    r: Ray,
    world: &HittableList,
    depth: i32,
    rng: &mut RandomNumberGenerator,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let scatter_result = rec.material.scatter(rng, r, &rec);
        if scatter_result.0 {
            let attenuation = scatter_result.1.unwrap();
            let scattered = scatter_result.2.unwrap();
            let recursed_color = ray_color(i, j, scattered, world, depth - 1, rng);
            // eprintln!(
            //     "> Scatter {} {} dir={} color= {}, {}, {}",
            //     i,
            //     j,
            //     r.direction,
            //     attenuation,
            //     recursed_color,
            //     attenuation * recursed_color
            // );
            return attenuation * recursed_color;
        }

        // eprintln!("> Diffuse {} {} = {}", i, j, rec.material.diffuse());
        return rec.material.diffuse();
    }

    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let blue = Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    let sky = white * (1.0 - t) + blue * t;

    // eprintln!("> Sky {} {} = {}", i, j, sky);
    return sky;
}

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);

    let r = (scale * pixel_color.x).sqrt();
    let g = (scale * pixel_color.y).sqrt();
    let b = (scale * pixel_color.z).sqrt();

    let ri = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let gi = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let bi = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    // eprintln!("write_color {} r {} g {} b {} ri {} gi {} bi {}", pixel_color, r, g, b, ri, gi, bi);

    println!("{} {} {}", ri, gi, bi);
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 10.0;
    let image_width: i32 = 1920;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel_x: i32 = 16;
    let samples_per_pixel_y: i32 = 16;
    let max_depth: i32 = 50;

    eprintln!(
        "Rendering {}x{} image with {}x{} samples per pixel",
        image_width, image_height, samples_per_pixel_x, samples_per_pixel_y
    );

    let mut rng = RandomNumberGenerator::create();
    rng.random_double();

    let args: Vec<String> = env::args().collect();

    let world = if args.iter().any(|i| i == "simple") {
        eprintln!("Loading simple scene");
        simple_scene()
    } else {
        eprintln!("Loading random scene");
        random_scene(&mut rng)
    };

    let lookfrom = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let vfov: f64 = 20.0; // vertical field-of-view in degrees
    let focus_dist: f64 = 10.0;
    let aperture: f64 = 0.1;

    let cam: Camera = Camera::create(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..image_width {
            let mut pixel_color = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for s in 0..samples_per_pixel_x * samples_per_pixel_y {
                let y = ((s / samples_per_pixel_y) as f64) / (samples_per_pixel_y as f64) - 0.5;
                let x = ((s % samples_per_pixel_y) as f64) / (samples_per_pixel_x as f64) - 0.5;
                let u = (i as f64 + x) / (image_width as f64 - 1.0);
                let v = (j as f64 + y) / (image_height as f64 - 1.0);
                let r = cam.get_ray(&mut rng, u, v);
                pixel_color += ray_color(i, j, r, &world, max_depth, &mut rng);
            }

            // eprintln!("> Pixel {},{} color {}", i, j, pixel_color);

            write_color(pixel_color, samples_per_pixel_x * samples_per_pixel_y);
        }
    }
    eprintln!("\nDone");
}
