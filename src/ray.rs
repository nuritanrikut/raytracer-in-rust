use super::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        return self.origin + self.direction * t;
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }
}
