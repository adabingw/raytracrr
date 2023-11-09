use std::{sync::Arc, ops::Range};

use crate::{vec::Vec3, ray::Ray};
use super::{Hit, hit_record::HitRecord, aabb::AABB};

/**
 * think of moving the incident ray backwards the offset amount, 
 * determining if an intersection occurs, 
 * and then moving that intersection point forward the offset amount.
 * 
 * need to move the intersection point forward the offset amount so that the intersection is 
 * actually in the path of the incident ray. If we forgot to move the intersection point forward 
 * then the intersection would be in the path of the offset ray, which isn't correct. 
 */

pub struct Translate {
    object: Arc<dyn Hit>,
    offset: Vec3
}

impl Translate {
    pub fn new(object: Arc<dyn Hit>, offset: Vec3) -> Translate {
        Translate {
            object, offset
        }
    }

    pub fn new_arc(object: Arc<dyn Hit>, offset: Vec3) -> Arc<dyn Hit> {
        Arc::new(Translate::new(object, offset))
    }
}

impl Hit for Translate {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction());
        self.object.hit(&moved_ray, time_range).map(|rec| {
            HitRecord {
                front_face: rec.front_face,
                p: rec.p + self.offset,
                normal: rec.normal, 
                t: rec.t, 
                u: rec.u,
                v: rec.v, 
                material: rec.material
            }
        })
    }

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        AABB::new(
            self.bounding_box(time_range.clone()).get_minimum() + self.offset,
            self.bounding_box(time_range).get_maximum() + self.offset
        )
    }
}
