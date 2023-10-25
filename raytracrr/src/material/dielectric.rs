use rand::{Rng};

use crate::material::{Scatter};
use crate::vec::{Colour, Vec3};
use crate::ray::{Ray};
use crate::hit::{HitRecord};

// with dielectric spheres is to note that if you use a negative radius, the geometry is unaffected, 
// but the surface normal points inward. This can be used as a bubble to make a hollow glass sphere:

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    // ir: index of refraction
    pub fn new(ir: f64) -> Dielectric {
        Dielectric {
            ir
        }
    }

    // real glass has reflectivity depending on the angle
    pub fn reflectance(cosine: f64, reflective_index: f64) -> f64 {
        // use schlick's approximation for reflectance
        let r_0 = ((1.0 - reflective_index) / (1.0 + reflective_index)).powi(2);
        r_0 + (1.0 - r_0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalized();

        // when the ray is in the material with the higher refractive index, there is not always be a 
        // solution to Snell’s law within the real numbers, and thus there is no refraction possible. 
        let cos_theta = ((-1.0) * unit_direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_reflect = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_reflect || will_reflect {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, refraction_ratio)
        };

        let scattered = Ray::new_(record.p, direction, r_in.time);

        Some((Colour::new(1.0, 1.0, 1.0), scattered))
    }
}
