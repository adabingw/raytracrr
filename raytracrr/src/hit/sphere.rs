use std::ops::Range;
use std::sync::Arc;

use super::aabb::{AABB};
use crate::material::Scatter;
use crate::ray::{Ray};
use crate::hit::{Hit, HitRecord};
use crate::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center, 
            radius,
            material
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        // (a + tb - C) ^ 2 = r^2, where P(t) = a + tb
        // check if ray hits the sphere, using quadratic equation
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        // find nearest root that falls in the accepted range
        // most ray tracers have found it convenient to add a valid interval for hits tmin to tmax
        //  hit only "counts" if tmin<t<tmax
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-b - sqrt_discriminant) / (2.0 * a);
        if !time_range.contains(&root) {
            root = (-b + sqrt_discriminant) / (2.0 * a);
            if !time_range.contains(&root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut record = HitRecord {
            p,
            t: root, 
            material: self.material.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        return Some(record);
    }

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let bbox = AABB::new(self.center - rvec, self.center + rvec);
        bbox
    }
}
