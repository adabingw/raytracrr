use crate::material::{Scatter};
use crate::vec::{Colour, Vec3};
use crate::ray::{Ray};
use crate::hit::{HitRecord};

pub struct Metal {
    albedo: Colour,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction().reflect(record.normal).normalized();
        let scattered = Ray::new(
            record.p, 
            reflected + self.fuzz * Vec3::random_in_sphere()
        );
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
