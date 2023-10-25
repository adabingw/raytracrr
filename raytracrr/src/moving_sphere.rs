use std::sync::Arc;

use super::material::Scatter;
use super::ray::{Ray};
use super::hit::{Hit, HitRecord};
use super::vec::{Point3, Vec3};

pub struct MovingSphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
    center_vec: Vec3
}

impl MovingSphere {
    pub fn new(center: Point3, center1: Point3, radius: f64, material: Arc<dyn Scatter>) -> MovingSphere {
        MovingSphere {
            center, 
            center_vec: center1 - center,
            radius,
            material
        }
    }

    pub fn sphere_center(&self, time: f64) -> Point3 {
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t = 1 yields center2.
        self.center + time * self.center_vec
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // (a + tb - C) ^ 2 = r^2, where P(t) = a + tb
        // check if ray hits the sphere, using quadratic equation
        let center = self.sphere_center(r.time);
        let oc = r.origin() - center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        // find nearest root that falls in the accepted range
        // most ray tracers have found it convenient to add a valid interval for hits tmin to tmax
        //  hit only "counts" if tmin < t < tmax
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-b - sqrt_discriminant) / (2.0 * a);
        if root < t_min || root > t_max {
            root = (-b + sqrt_discriminant) / (2.0 * a);
            if root < t_min || root > t_max {
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
}
