use crate::material::{Scatter};
use crate::vec::{Colour};
use crate::ray::{Ray};
use crate::hit::{HitRecord};

pub struct Metal {
    albedo: Colour
}

impl Metal {
    pub fn new(albedo: Colour) -> Metal {
        Metal {
            albedo
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction().reflect(record.normal).normalized();
        let scattered = Ray::new(record.p, reflected);
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
