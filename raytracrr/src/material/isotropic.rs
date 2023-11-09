use std::sync::Arc;
use crate::{texture::Texture, ray::Ray, hit::hit_record::HitRecord, vec::{Colour, Vec3}};
use super::Scatter;

pub struct Isotropic {
    albedo: Arc<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }

    pub fn new_arc(albedo: Arc<dyn Texture>) -> Arc<Isotropic> {
        Arc::new(Isotropic::new(albedo))
    }
}

impl Scatter for Isotropic {
    // picks a uniform random direction:
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let scattered = Ray::new_(
            record.p, Vec3::random_in_sphere(), r_in.time
        );
        let attenuation = self.albedo.value(record.u, record.v, record.p);
        Some((attenuation, scattered))
    }
}
