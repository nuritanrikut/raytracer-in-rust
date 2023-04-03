use std::{fmt, ops};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn add_assign(&mut self, b: Vec3) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:+.4}, {:+.4}, {:+.4})", self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        };
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, b: Vec3) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: f64) -> Vec3 {
        return Vec3 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, b: f64) -> Vec3 {
        return Vec3 {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
        };
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn near_zero(v: Vec3) -> bool {
    let s: f64 = 1e-8;
    return (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s);
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - n * dot(v, n) * 2.0;
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}
