use super::vec3::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub struct RandomNumberGenerator {
    pub state: u64,
    pub div: u64,
    pub modulo: u64,
    pub small_rng: SmallRng,
}

impl RandomNumberGenerator {
    pub fn create() -> RandomNumberGenerator {
        return RandomNumberGenerator {
            state: 675248,
            div: 1000,
            modulo: 1000000,
            small_rng: SmallRng::seed_from_u64(1),
        };
    }

    #[allow(dead_code)]
    fn next(&mut self) -> u64 {
        self.state = self.state * self.state / self.div % self.modulo;
        return self.state;
    }

    pub fn random_double(&mut self) -> f64 {
        // use this version if you want actual PRNG
        // return self.small_rng.gen();

        // use this version if you want repeatable sequence of numbers
        let t1 = self.next();
        let t2 = self.next();
        return ((t1 * self.modulo + t2) as f64) / (self.modulo as f64) / (self.modulo as f64);
    }

    pub fn random_range(&mut self, min: f64, max: f64) -> f64 {
        return min + (max - min) * self.random_double();
    }

    pub fn random_vec3(&mut self) -> Vec3 {
        return Vec3 {
            x: self.random_double(),
            y: self.random_double(),
            z: self.random_double(),
        };
    }

    pub fn random_vec3_range(&mut self, min: f64, max: f64) -> Vec3 {
        return Vec3 {
            x: self.random_range(min, max),
            y: self.random_range(min, max),
            z: self.random_range(min, max),
        };
    }

    pub fn random_in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let p = self.random_vec3_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector(&mut self) -> Vec3 {
        return unit_vector(self.random_in_unit_sphere());
    }

    pub fn random_in_unit_disk(&mut self) -> Vec3 {
        loop {
            let p = Vec3 {
                x: self.random_range(-1.0, 1.0),
                y: self.random_range(-1.0, 1.0),
                z: 0.0,
            };
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}
