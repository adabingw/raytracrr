use core::time;
use std::{ops::Range, sync::Arc};

use crate::material::Scatter;

use super::{Hit, aabb::AABB, hit_record::HitRecord};
use crate::vec::{Vec3};

/**
 * 2D rectable rendered in the 3D plane
 * u and v represents the two ranges of the 2D shape along 2 axis
 * k represents the offset of the 3rd axis
 * axis represents our annotation of which axis it is parallel with
 */
pub struct Rect {
    u: Range<f64>, 
    v: Range<f64>, 
    k: f64, 
    axis: i32, // xy: 0, xz: 1, yz: 2
    material: Arc<dyn Scatter> 
}

impl Rect {
    pub fn new(u: Range<f64>, v: Range<f64>, k: f64, axis: i32, material: Arc<dyn Scatter>) -> Rect {
        Rect {
            u, v, k, axis, material
        }
    }
}

impl Hit for Rect {
    fn hit(&self, r: &crate::ray::Ray, time_range: Range<f64>) -> Option<HitRecord> {

        // finding the t in R(t) = At + B
        let t = if self.axis == 0 { // xy
            (self.k - r.origin().z()) / r.direction().z()
        } else if self.axis == 1 { // xz
            (self.k - r.origin().y()) / r.direction().y()
        } else { // yz
            (self.k - r.origin().x()) / r.direction().x()
        };

        // no hit if hit point t outside of time interval
        if !time_range.contains(&t) {
            return None;
        }

        // finding the coords for which ray hits rect
        let a = if self.axis == 0 { // xy
            r.origin().x() + t * r.direction().x()
        } else if self.axis == 1 { // xz
            r.origin().x() + t * r.direction().x()
        } else { // yz
            r.origin().y() + t * r.direction().y()
        };

        let b = if self.axis == 0 { // xy
            r.origin().y() + t * r.direction().y()
        } else if self.axis == 1 { // xz
            r.origin().z() + t * r.direction().z()
        } else { // yz
            r.origin().z() + t * r.direction().z()
        };

        // hit point must be contained in the quad defined by u and v
        if !(self.u.contains(&a) && self.v.contains(&b)) {
            return None;
        }

        // naming convention a bit confusing lol
        let i = (a - self.u.start) / (self.u.end - self.u.start);
        let j = (b - self.v.start) / (self.v.end - self.v.start);

        // normal should be pointing parallel to the 3rd axis
        let outward_normal = if self.axis == 0 { // xy
            Vec3::new(0.0, 0.0, 1.0)
        } else if self.axis == 1 { // xz
            Vec3::new(0.0, 1.0, 0.0)
        } else { // yz
            Vec3::new(1.0, 0.0, 0.0)
        };

        let p = r.at(t);
        let material = Arc::clone(&self.material);

        let mut record = HitRecord {
            p, 
            normal: outward_normal, 
            t, 
            u: i, 
            v: j, 
            material,
            front_face: false
        };
        record.set_face_normal(r, outward_normal);
        Some(record)
    }

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        let minimum = if self.axis == 0 {
            Vec3::new(self.u.start, self.v.start, self.k - 0.0001)
        } else if self.axis == 1 {
            Vec3::new(self.u.start, self.k - 0.0001, self.v.start)
        } else {
            Vec3::new(self.k - 0.0001, self.u.start, self.v.start)
        };
        let maximum = if self.axis == 0 {
            Vec3::new(self.u.end, self.v.end, self.k - 0.0001)
        } else if self.axis == 1 {
            Vec3::new(self.u.end, self.k - 0.0001, self.v.end)
        } else {
            Vec3::new(self.k - 0.0001, self.u.end, self.v.end)
        };
        AABB::new(minimum, maximum)
    }
}
