use super::material::*;
use super::ray::*;
use super::rng::*;
use super::vec3::*;
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Copy, Clone)]
struct NullMaterial {}

impl Material for NullMaterial {
    fn diffuse(&self) -> Color {
        return Color::default();
    }

    fn scatter(
        &self,
        _: &mut RandomNumberGenerator,
        _: Ray,
        _: &HitRecord,
        _: &mut Color,
        _: &mut Ray
    ) -> bool {
        return false;
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(NullMaterial {}) as Rc<dyn Material>,
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            p: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material: Rc::new(NullMaterial {}) as Rc<dyn Material>,
            t: 0.0,
            front_face: false,
        };
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn clone(&self) -> HitRecord {
        return HitRecord {
            p: self.p,
            normal: self.normal,
            material: self.material.clone(),
            t: self.t,
            front_face: self.front_face,
        };
    }
}
