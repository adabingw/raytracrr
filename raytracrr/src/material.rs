pub mod metal;
pub mod matte;

use super::vec::{Colour};
use super::hit::{HitRecord};
use super::ray::{Ray};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Colour, Ray)>;
}
