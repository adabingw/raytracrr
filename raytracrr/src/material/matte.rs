use crate::material::{Scatter};
use crate::vec::{Colour, Vec3};
use crate::ray::{Ray};
use crate::hit::{HitRecord};

pub struct Matte {
    albedo: Colour
}

impl Matte {
    pub fn new(albedo: Colour) -> Matte {
        Matte {
            albedo
        }
    }
}

// it can either scatter always and attenuate by its reflectance R
// or it can scatter with no attenuation but absorb the fraction 1−R of the rays, 
// or it could be a mixture of those strategies.
impl Scatter for Matte {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        // If the random unit vector is exactly opposite the normal vector, 
        // the two will sum to zero, which will result in a zero scatter direction vector.
        let mut scatter_direction = record.p + record.normal + Vec3::random_in_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        let scattered = Ray::new(record.p, scatter_direction);
        return Some((self.albedo, scattered));
    }
}