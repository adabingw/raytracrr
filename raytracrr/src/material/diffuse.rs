use std::sync::Arc;

use crate::{texture::{Texture, solid::Solid}, vec::{Vec3, Colour}};
use super::Scatter;

pub struct Diffuse {
    emit: Arc<dyn Texture>
}

impl Diffuse {
    pub fn new(emit: Arc<dyn Texture>) -> Diffuse {
        Diffuse { emit }
    }

    pub fn new_diffuse(c: Colour) -> Diffuse {
        Diffuse { 
            emit: Arc::new(Solid::new(c))
        }
    }
}

impl Scatter for Diffuse {
    fn scatter(&self, r_in: &crate::ray::Ray, record: &crate::hit::hit_record::HitRecord) -> Option<(crate::vec::Colour, crate::ray::Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Colour {
        self.emit.value(u, v, p)
    }
}
