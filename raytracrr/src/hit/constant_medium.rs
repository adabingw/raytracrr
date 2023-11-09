use std::{sync::Arc, ops::Range};
use rand::prelude::*;

use crate::{material::{Scatter, isotropic::Isotropic}, texture::Texture, ray::Ray, vec::Vec3};
use super::{Hit, hit_record::HitRecord, aabb::AABB};

/**
 * As the ray passes through the volume, it may scatter at any point. 
 * The denser the volume, the more likely that is. 
 * The probability that the ray scatters in any small distance ΔL is:
 *                                  probability = C ⋅ ΔL
 * where C is proportional to the optical density of the volume. 
 * If you go through all the differential equations, for a random number you get a distance where the 
 * scattering occurs. If that distance is outside the volume, then there is no “hit”. 
 * For a constant volume we just need the density C and the boundary.
 */
pub struct ConstantMedium {
    boundary: Arc<dyn Hit>,
    phase_function: Arc<dyn Scatter>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hit>, albedo: Arc<dyn Texture>, density: f64) -> ConstantMedium {
        ConstantMedium { 
            boundary, 
            phase_function: Arc::new(Isotropic::new(albedo)), 
            neg_inv_density: -1.0 / density
        }
    }

    pub fn new_arc(boundary: Arc<dyn Hit>, albedo: Arc<dyn Texture>, density: f64) -> Arc<Box<dyn Hit>> {
        Arc::new(Box::new(ConstantMedium::new(boundary, albedo, density)))
    }
}

impl Hit for ConstantMedium {
    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        self.boundary.bounding_box(time_range)
    }

    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        if let Some(rec1) = self.boundary.hit(r, -f64::INFINITY..f64::INFINITY) {
            if let Some(rec2) = self.boundary.hit(r, (rec1.t + 0.0001)..f64::INFINITY) {
                let t1 = rec1.t.max(time_range.start);
                let t2 = rec2.t.min(time_range.end);

                if t1 < t2 {
                    let t1 = t1.max(0.0);

                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (t2 - t1) * ray_length;
                    let hit_distance = self.neg_inv_density * thread_rng().gen::<f64>().ln();

                    // make sure hit is inside boundary, otherwise no hit
                    if hit_distance <= distance_inside_boundary {
                        let t = t1 + hit_distance / ray_length;
                        let p = r.at(t);

                        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary.
                        let u = 0.0; // arbitrary.
                        let v = 0.0; // arbitrary.

                        return Some(HitRecord {
                            p,
                            normal,
                            t,
                            u,
                            front_face: true,
                            v,
                            material: Arc::clone(&self.phase_function),
                        });
                    }
                }
            }
        }

        None
    }
}
