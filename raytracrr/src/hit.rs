pub mod sphere;
pub mod moving_sphere;
pub mod aabb;
pub mod world;
pub mod bvh;
pub mod hit_record;
pub mod quad;

use super::vec::{Point3, Vec3};
use super::ray::{Ray};
use super::material::{Scatter};
use aabb::{AABB};
use std::ops::Range;
use std::sync::{Arc};
use hit_record::{HitRecord};

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord>;

    // interval values constructed without arguments will be empty by default. 
    // Since an aabb object has an interval for each of its three dimensions, 
    // each of these will then be empty by default, and therefore aabb objects will be empty by default. 
    // Thus, some objects may have empty bounding volumes. 
    // For example, consider a hittable_list object with no children.
    // recall that some objects may be animated. 
    // Such objects should return their bounds over the entire range of motion, from time=0 to time=1.
    fn bounding_box(&self, time_range: Range<f64>) -> AABB;
}
