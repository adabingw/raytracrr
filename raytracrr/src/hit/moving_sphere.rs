use std::f64::consts::PI;
use std::ops::Range;
use std::sync::Arc;

use super::aabb::{AABB};
use crate::material::Scatter;
use crate::ray::{Ray};
use crate::hit::{Hit, HitRecord};
use crate::vec::{Point3, Vec3};

pub struct MovingSphere {
    centers: (Point3, Point3),
    radius: f64,
    material: Arc<dyn Scatter>
}

impl MovingSphere {
    pub fn new(center: Point3, center1: Point3, radius: f64, material: Arc<dyn Scatter>) -> MovingSphere {
        MovingSphere {
            centers: (center, center1), 
            radius,
            material
        }
    }

    pub fn sphere_center(&self, time: f64) -> Point3 {
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t = 1 yields center2.
        self.centers.0 + time * self.centers.1
    }

    pub fn new_arc(center: Point3, center1: Point3, radius: f64, material: Arc<dyn Scatter>) -> Arc<Box<MovingSphere>> {
        Arc::new(Box::new(MovingSphere::new(center, center1, radius, material)))
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
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
        if !time_range.contains(&root) {
            root = (-b + sqrt_discriminant) / (2.0 * a);
            if !time_range.contains(&root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - center) / self.radius;
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

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        // we want the bounds of its entire range of motion. 
        // we can take the box of the sphere at time=0, and the box of the sphere at time=1, 
        // and compute the box around those two boxes.
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let centre0 = self.sphere_center(time_range.start);
        let minimum0 = centre0 - rvec;
        let maximum0 = centre0 + rvec;
        let box0 = AABB::new(minimum0, maximum0);

        let centre1 = self.sphere_center(time_range.end);
        let minimum1 = centre1 - rvec;
        let maximum1 = centre1 + rvec;
        let box1 = AABB::new(minimum1, maximum1);

        AABB::surrounding_box(box0, box1)
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centred at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let phi = p.y().atan2(p.x()) + PI;
    let theta = -p.y().acos();

    let u = phi / (2.0 * PI);
    let v = theta / PI;

    (u, v)
}
