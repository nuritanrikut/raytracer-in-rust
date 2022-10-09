use super::hit_record::*;
use super::material::*;
use super::ray::*;
use super::rng::*;
use super::vec3::*;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn diffuse(&self) -> Color {
        return self.albedo;
    }

    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        _: Ray,
        rec: &HitRecord,
    ) -> (bool, Option<Color>, Option<Ray>) {
        let mut scatter_direction = rec.normal + rng.random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;
        return (true, Some(attenuation), Some(scattered));
    }
}
