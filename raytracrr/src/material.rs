pub mod metal;
pub mod matte;
pub mod dielectric;
pub mod diffuse;
pub mod isotropic;

use super::vec::{Vec3, Colour};
use super::hit::hit_record::{HitRecord};
use super::ray::{Ray};

pub trait Scatter : Send + Sync {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)>;

    // just tells the ray what color it is and performs no reflection
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Colour {
        Colour::new(0.0, 0.0, 0.0)
    }
}
