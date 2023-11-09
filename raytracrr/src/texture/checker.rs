use crate::vec::{Colour, Vec3};
use crate::texture::{Texture};
use super::solid::{Solid};
use std::sync::{Arc};

// solid (spatial) texture depends only on the position of each point in 3D space. 
// think of a solid texture as if it's coloring all of the points in space itself, 
// instead of coloring a given object in that space. 
// object can move through the colors of the texture as it changes position, 

// since a spatial texture function is driven by a given position in space, 
// the texture value() function ignores the u and v parameters, and uses only the p parameter.

// first compute the floor of each component of the input point. 
// could truncate the coordinates, but that would pull values toward zero, 
// which would give us the same color on both sides of zero. 
// The floor function will always shift values to the integer value on the left (toward negative infinity). 
// Given these three integer results (⌊x⌋,⌊y⌋,⌊z⌋) we take their sum and compute the result modulo two, 
// which gives us either 0 or 1. Zero maps to the even color, and one to the odd color.

pub struct Checker {
    scale: f64, 
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Checker {
        Checker {
            scale, 
            even, 
            odd
        }
    }

    pub fn new_texture(scale: f64, c1: Colour, c2: Colour) -> Checker {
        Checker {
            scale, 
            even: Arc::new(Solid::new(c1)), 
            odd: Arc::new(Solid::new(c2))
        }
    }

    pub fn new_arc(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Arc<Checker> {
        Arc::new(Checker::new(scale, even, odd))
    }

    pub fn new_texture_arc(scale: f64, c1: Colour, c2: Colour) -> Arc<Checker> {
        Arc::new(Checker::new_texture(scale, c1, c2))
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        let x = (self.scale * p.x()) as i32;
        let y = (self.scale * p.y()) as i32;
        let z = (self.scale * p.z()) as i32;

        let is_even = (x + y + z) % 2 == 0;
        
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
