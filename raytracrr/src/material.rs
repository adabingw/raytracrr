pub mod metal;
pub mod matte;
pub mod dielectric;

use super::vec::{Colour};
use super::hit::hit_record::{HitRecord};
use super::ray::{Ray};

pub trait Scatter : Send + Sync {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)>;
}
