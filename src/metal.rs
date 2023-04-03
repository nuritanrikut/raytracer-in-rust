use super::hit_record::*;
use super::material::*;
use super::ray::*;
use super::rng::*;
use super::vec3::*;

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn diffuse(&self) -> Color {
        return self.albedo;
    }

    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected + rng.random_in_unit_sphere() * self.fuzz,
        };
        *attenuation = self.albedo;
        return dot(scattered.direction, rec.normal) > 0.0;
    }
}
