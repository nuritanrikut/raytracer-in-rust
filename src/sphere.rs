use super::hit_record::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t_max < t {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t_max < t {
                return false;
            }
        }

        rec.t = t;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();

        return true;
    }
}
