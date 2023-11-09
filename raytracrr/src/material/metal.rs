use std::sync::Arc;

use crate::material::{Scatter};
use crate::texture::Texture;
use crate::vec::{Colour, Vec3};
use crate::ray::{Ray};
use crate::hit::hit_record::{HitRecord};

pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz
        }
    }

    pub fn new_arc(albedo: Arc<dyn Texture>, fuzz: f64) -> Arc<Metal> {
        Arc::new(Metal::new(albedo, fuzz))
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction().reflect(record.normal).normalized();
        let attenuation = self.albedo.value(record.u, record.v, record.p);
        let scattered = Ray::new_(
            record.p, 
            reflected + self.fuzz * Vec3::random_in_sphere(),
            r_in.time
        );
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
