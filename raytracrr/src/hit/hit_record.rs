use crate::vec::{Point3, Vec3};
use crate::ray::{Ray};
use crate::material::{Scatter};
use std::sync::{Arc};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatter>,
    pub t: f64,
    pub u: f64, // surface coord of ray obj hit point
    pub v: f64, // surface coord of ray obj hit point
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            // ray is outside the object
            outward_normal
        } else {
            // ray is inside the object
            (-1.0) * outward_normal
        }
    }
}
