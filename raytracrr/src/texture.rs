pub mod solid;
pub mod checker;
pub mod image;
pub mod noise;

use crate::vec::{Colour, Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour;
}

// process of applying a material effect to an object in the scene. 
// The "texture" part is the effect, and the "mapping" part is in the mathematical 
// sense of mapping one space onto another. 
// This effect could be any material property: color, shininess, bump geometry (called Bump Mapping), 
// or even material existence (to create cut-out regions of the surface).

// The most common type of texture mapping maps an image onto the surface of an object, 
// defining the color at each point on the object’s surface. 
// In practice, we implement the process in reverse: given some point on the object, 
// we’ll look up the color defined by the texture map.

// we'll make the texture colors procedural, and will create a texture map of constant color.

// in order to perform the texture lookup, need a texture coordinate.
// for now, pass in two dimensional texture coordinates. By convention, texture coordinates are named u and v. 
// For a constant texture, every (u,v) pair yields a constant color, actually ignore the coordinates completely. 
// However, other texture types will need these coordinates, so we keep these in the method interface.

// The primary method of texture classes is the color value(...) method, 
// which returns the texture color given the input coordinates. 
// In addition to taking the point's texture coordinates u and v, 
// we also provide the position of the point in question, for reasons that will become apparent later.
