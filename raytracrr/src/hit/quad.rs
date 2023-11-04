use std::{sync::Arc, ops::Range};
use crate::{vec::{Point3, Vec3}, material::Scatter};
use super::{aabb::AABB, Hit, hit_record::HitRecord};

/**
 *    v ------- Q + u + v
 *   /         /
 *  /         /
 * Q --------u
 * 
 * 1. find plane containing quad
 * 2. solve for intersection of ray and quad containing plane
 * 3. determine if hit point lies inside quad
 * 
 * 2 -
 * formula for a plane: Ax + By + Cz = D
 * n = (A, B, C) -> normal vector
 * v = (x, y, z) -> position vector
 * n.dot(v) = D = Ax + By + Cz 
 * finding intersection with ray: R(t) = P + td 
 * t = (D - n.dot(P)) / (n.dot(d))
 * 
 * 1 - 
 * n = (u.cross(v)).normalized()
 * plane is all points (x, y, z) satisfying Ax + By + Cz = D
 * Q lies on plane, so we can 
 * n.dot(Q) = D
 * 
 * 3 - 
 * on plane, but is it on quad?
 * for arbitrary P, P = Q + au + bv
 * p = P - Q = au + bv (p = vector from Q to P)
 * 
 * w = n / (n.dot(n))
 * a = v.cross(p).dot(n)
 * b = u.cross(p).dot(n)
 * 
 * check that 0 <= a <= 1 and 0 <= b <= 1
 */

// TODO: fix wack
pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3, 
    material: Arc<dyn Scatter>,
    b: AABB,
    normal: Vec3, 
    D: f64, 
    w: Vec3
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Scatter>) -> Quad {
        let n = u.cross(v).normalized();
        let D = n.dot(Q);
        let w = n / n.dot(n);
        Quad {
            Q, 
            u, 
            v, 
            material, 
            b: AABB::new_pad(Q, Q + u + v), 
            normal: n, 
            D: D,
            w: w
        }
    }
}

impl Hit for Quad {
    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        self.b.clone()
    }

    fn hit(&self, r: &crate::ray::Ray, time_range: std::ops::Range<f64>) -> Option<HitRecord> {
        let n_dot_d = self.normal.dot(r.direction());

        // no hit if ray parallel to plane
        if n_dot_d.abs() < 1.0e-8 {
            return None;
        }

        let t = (self.D - self.normal.dot(r.origin())) / n_dot_d;

        if !time_range.contains(&t) {
            return None;
        }

        // determining if hit point is in planar shape
        let intersection = r.at(t);
        let p_vec = intersection - self.Q;

        let alpha = self.w.dot(p_vec.cross(self.v));
        let beta = self.w.dot(self.u.cross(p_vec));

        if (alpha < 0.0 ) || (beta < 0.0) || (alpha > 1.0) || (beta) > 1.0 {
            return None;
        }

        let mut record = HitRecord {
            p: intersection,
            t: t, 
            u: alpha, 
            v: beta,
            material: self.material.clone(),
            normal: self.normal,
            front_face: false
        };

        record.set_face_normal(r, self.normal);

        return Some(record);
    }
}
