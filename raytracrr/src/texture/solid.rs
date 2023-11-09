use std::sync::Arc;

use crate::vec::{Vec3, Colour};
use crate::texture::{Texture};

pub struct Solid {
    colour_value: Colour
}

impl Solid {
    pub fn new(colour_value: Colour) -> Solid {
        Solid {
            colour_value
        }
    }

    pub fn new_arc(colour_value: Colour) -> Arc<Solid> {
        Arc::new(Solid::new(colour_value))
    }
}

impl Texture for Solid {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        self.colour_value
    }
}
