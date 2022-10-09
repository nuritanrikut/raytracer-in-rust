use super::ray::*;
use super::utils::*;
use super::rng::*;
use super::vec3::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn create(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = unit_vector(lookfrom - lookat);
        let u: Vec3 = unit_vector(cross(vup, w));
        let v: Vec3 = cross(w, u);

        let origin: Vec3 = lookfrom;
        let horizontal: Vec3 = u * (focus_dist * viewport_width);
        let vertical: Vec3 = v * (focus_dist * viewport_height);
        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius: f64 = aperture / 2.0;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        };
    }

    pub fn get_ray(self, rng: &mut RandomNumberGenerator, s: f64, t: f64) -> Ray {
        let rd = rng.random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        };
    }
}
