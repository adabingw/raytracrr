use std::sync::Arc;

use crate::{perlin::Perlin, vec::{Colour, Vec3}};
use super::Texture;

pub struct Noise {
    perlin: Perlin,
    scale: f64
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            perlin: Perlin::new(),
            scale
        }
    }

    pub fn new_arc(scale: f64) -> Arc<Noise> {
        Arc::new(Noise::new(scale))
    }
}

impl Texture for Noise {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        // cast the perlin output back to between 0 and 1.
        // Colour::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.perlin.noise(p * self.scale))
        // Colour::new(1.0, 1.0, 1.0) * (self.perlin.turb(p * self.scale, 7))
        let s = self.scale * p;
        Colour::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z() + 10.0 * self.perlin.turb(s, 7)).sin())
    }
}
