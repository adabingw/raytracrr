use std::f64::consts::PI;
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
        let outward_normal = (p - self.center) / self.radius;
        
        let (u, v) = get_sphere_uv(outward_normal);
        let mut record = HitRecord {
            p,
            t: root, 
            u: u, 
            v: v,
            material: self.material.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        record.set_face_normal(r, outward_normal);

        return Some(record);
    }

    // bounding box is just the box that surrounds the sphere
    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let bbox = AABB::new(self.center - rvec, self.center + rvec);
        bbox
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centred at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let phi = (-p.z()).atan2(p.x()) + PI;
    let theta = (-p.y()).acos();

    let u = phi / (2.0 * PI);
    let v = theta / PI;

    (u, v)
}
