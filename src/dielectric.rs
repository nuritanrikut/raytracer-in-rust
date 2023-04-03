use super::hit_record::*;
use super::material::*;
use super::ray::*;
use super::rng::*;
use super::vec3::*;

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}

impl Material for Dielectric {
    fn diffuse(&self) -> Color {
        return Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
    }

    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction);
        let d = dot(-unit_direction, rec.normal);
        let cos_theta = d.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let t = rng.random_double();
        let refl = reflectance(cos_theta, refraction_ratio);
        let should_reflect = refl > t;
        // eprintln!(
        //     "> reflectance unit_direction( {:+.3}, {:+.3}, {:+.3} ) rec.normal( {:+.3}, {:+.3}, {:+.3} ) d {:+.3} cos_theta {:+.3} refraction_ratio {:+.3} refl {:+.3} t {:+.3}",
        //     (-unit_direction).x,
        //     (-unit_direction).y,
        //     (-unit_direction).z,
        //     rec.normal.x,
        //     rec.normal.y,
        //     rec.normal.z,
        //     d,
        //     cos_theta,
        //     refraction_ratio,
        //     refl,
        //     t
        // );
        let direction = if cannot_refract || should_reflect {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: rec.p,
            direction,
        };
        return true;
    }
}
