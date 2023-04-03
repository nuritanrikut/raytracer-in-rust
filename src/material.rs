use super::hit_record::*;
use super::ray::*;
use super::rng::*;
use super::vec3::*;

pub trait Material {
    fn diffuse(&self) -> Color;
    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray
    ) -> bool;
}
