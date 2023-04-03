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
// use rayon::prelude::*;
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
    row: usize,
    col: usize,
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
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec
            .material
            .scatter(rng, r, &rec, &mut attenuation, &mut scattered)
        {
            let recursed_color = ray_color(row, col, scattered, world, depth - 1, rng);
            // eprintln!(
            //     "> Scatter {} {} dir={} attenuation={} recursed_color={} out={}",
            //     col,
            //     row,
            //     r.direction,
            //     attenuation,
            //     recursed_color,
            //     attenuation * recursed_color
            // );
            coz::progress!("scatter");
            return attenuation * recursed_color;
        }

        // eprintln!("> Diffuse {} {} = {}", col, row, rec.material.diffuse());
        coz::progress!("diffuse");
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

    // eprintln!("> Sky {} {} = {}", col, row, sky);
    coz::progress!("sky");
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

    // eprintln!(
    //     "write_color {} r {:+.4} g {:+.4} b {:+.4} ri {} gi {} bi {}",
    //     pixel_color, r, g, b, ri, gi, bi
    // );

    println!("{} {} {}", ri, gi, bi);
}

struct Job<'a> {
    pub rng: RandomNumberGenerator,
    pub row: usize,
    pub col: usize,
    pub world: &'a HittableList,
    pub camera: Camera,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel_x: i32,
    pub samples_per_pixel_y: i32,
    pub max_depth: i32,
}

unsafe impl Send for Job<'_> {}

fn render_job(job: &mut Job) -> Color {
    let mut color = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let width_minus_one = job.image_width as f64 - 1.0;
    let height_minus_one = job.image_height as f64 - 1.0;

    for sample_y in 0..job.samples_per_pixel_y {
        let y = (sample_y as f64) / (job.samples_per_pixel_y as f64) - 0.5;
        let v = (job.row as f64 + y) / height_minus_one;

        for sample_x in 0..job.samples_per_pixel_x {
            // eprintln!(
            //     "Job {} {}, sample {} {}",
            //     job.col, job.row, sample_x, sample_y
            // );

            let x = (sample_x as f64) / (job.samples_per_pixel_x as f64) - 0.5;
            let u = (job.col as f64 + x) / width_minus_one;
            let r = job.camera.get_ray(&mut job.rng, u, v);
            color.add_assign(ray_color(
                job.row,
                job.col,
                r,
                &job.world,
                job.max_depth,
                &mut job.rng,
            ));
        }
    }

    return color;
}

fn render(image_width: i32, image_height: i32, jobs: &mut [Job], pixels: &mut Vec<Vec<Color>>) {
    let job_count = image_width * image_height;
    jobs.iter_mut()
        .zip(0..job_count)
        .for_each(|(job, job_index): (&mut Job, i32)| {
            if job_index % image_width == 0 {
                let remaining_lines = (job_count - job_index) / image_width;
                eprint!("Lines remaining: {}  \r", remaining_lines);
            }
            pixels[job.row][job.col] = render_job(job);
        });

    eprint!("Lines remaining: 0  \n");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 10.0;
    const IMAGE_WIDTH: i32 = 1920;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL_X: i32 = 16;
    const SAMPLES_PER_PIXEL_Y: i32 = 16;
    const SAMPLE_COUNT: i32 = SAMPLES_PER_PIXEL_X * SAMPLES_PER_PIXEL_Y;
    const JOB_COUNT: i32 = IMAGE_HEIGHT * IMAGE_WIDTH;
    const MAX_DEPTH: i32 = 50;

    eprintln!(
        "Rendering {}x{} image with {}x{} samples per pixel",
        IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL_X, SAMPLES_PER_PIXEL_Y
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

    const LOOKFROM: Point3 = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    const LOOKAT: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    const VUP: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    const VFOV: f64 = 20.0; // vertical field-of-view in degrees
    const FOCUS_DIST: f64 = 10.0;
    const APERTURE: f64 = 0.1;

    let camera: Camera = Camera::create(
        LOOKFROM,
        LOOKAT,
        VUP,
        VFOV,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DIST,
    );

    let mut pixels: Vec<Vec<Color>> = (0..IMAGE_HEIGHT)
        .map(|_| {
            (0..IMAGE_WIDTH)
                .map(|_| Color::default())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut jobs: Vec<Job> = Vec::with_capacity(JOB_COUNT as usize);

    for row in 0usize..(IMAGE_HEIGHT as usize) {
        // eprint!("Creating jobs for line: {}  \r", row);
        for col in 0usize..(IMAGE_WIDTH as usize) {
            let job = Job {
                rng: rng.clone(),
                row,
                col,
                world: &world,
                camera,
                image_width: IMAGE_WIDTH,
                image_height: IMAGE_HEIGHT,
                samples_per_pixel_x: SAMPLES_PER_PIXEL_X,
                samples_per_pixel_y: SAMPLES_PER_PIXEL_Y,
                max_depth: MAX_DEPTH,
            };
            jobs.push(job);
        }
    }

    // eprint!("\n");
    eprintln!("Created {} jobs", JOB_COUNT);

    render(IMAGE_WIDTH, IMAGE_HEIGHT, jobs.as_mut_slice(), &mut pixels);

    eprint!("Jobs finished\n");
    eprint!("Writing image\n");

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for row in (0usize..(IMAGE_HEIGHT as usize)).rev() {
        for col in 0usize..(IMAGE_WIDTH as usize) {
            write_color(pixels[row][col], SAMPLE_COUNT);
        }
    }
    eprintln!("\nDone");
}
