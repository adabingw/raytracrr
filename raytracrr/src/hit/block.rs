use std::{sync::Arc, ops::Range};

use crate::{vec::Vec3, material::Scatter, hit::rect::Rect, ray::Ray};
use super::{Hit, aabb::AABB, hit_record::HitRecord};

// ROTATIONS AND TRANSLATIONS
// In ray tracing, usually done with an instance. 
// An instance is a copy of a geometric primitive that has been placed into the scene. 
// This instance is entirely independent of the other copies of the primitive and can be moved or rotated. 
// In this case, our geometric primitive is our hittable box object, and we want to rotate it. 
// This is especially easy in ray tracing because we don’t actually need to move objects in the scene; 
// instead we move the rays in the opposite direction. For example, consider a translation (often called a move). 
// We could take the pink box at the origin and add two to all its x components, 
// or (as we almost always do in ray tracing) leave the box where it is, 
// but in its hit routine subtract two off the x-component of the ray origin.

pub struct Block {
    b_min: Vec3, 
    b_max: Vec3, 
    sides: Vec<Arc<dyn Hit>>
}

impl Block {
    pub fn new(b_min: Vec3, b_max: Vec3, material: Arc<dyn Scatter>) -> Block {
        let Vec3{e: [p0x, p0y, p0z]} = b_min;
        let Vec3{e: [p1x, p1y, p1z]} = b_max;

        let mut sides: Vec<Arc<dyn Hit>> = Vec::new();
        sides.push(Arc::new(Rect::new(p0x..p1x, p0y..p1y, p1z, 0, Arc::clone(&material))));
        sides.push(Arc::new(Rect::new(p0x..p1x, p0y..p1y, p0z, 0, Arc::clone(&material))));
        sides.push(Arc::new(Rect::new(p0x..p1x, p0z..p1z, p1y, 1, Arc::clone(&material))));
        sides.push(Arc::new(Rect::new(p0x..p1x, p0z..p1z, p0y, 1, Arc::clone(&material))));
        sides.push(Arc::new(Rect::new(p0y..p1y, p0z..p1z, p1x, 2, Arc::clone(&material))));
        sides.push(Arc::new(Rect::new(p0y..p1y, p0z..p1z, p0x, 2, material)));

        assert_eq!(sides.len(), 6);

        Block {
            b_min,
            b_max,
            sides,
        }
    }

    pub fn new_arc(b_min: Vec3, b_max: Vec3, material: Arc<dyn Scatter>) -> Arc<Box<dyn Hit>> {
        Arc::new(Box::new(Block::new(b_min, b_max, material)))
    }

}

impl Hit for Block {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        self.sides.hit(r, time_range)
    }

    fn bounding_box(&self, time_range: std::ops::Range<f64>) -> AABB {
        AABB::new(self.b_min, self.b_max)
    }
}

impl Hit for Vec<Arc<dyn Hit>> {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        self.as_slice().hit(r, time_range)
    }

    fn bounding_box(&self, tr: Range<f64>) -> AABB {
        self.as_slice().bounding_box(tr)
    }
}

impl Hit for [Arc<dyn Hit>] {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = time_range.end;

        for object in self {
            if let Some(record) = object.hit(r, time_range.start..closest_so_far) {
                closest_so_far = record.t;
                rec = Some(record);
            }
        }

        rec
    }

    fn bounding_box(&self, tr: Range<f64>) -> AABB {
        match self.len() {
            0 => AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            1 => self.first().unwrap().bounding_box(tr),
            _ => self
                .iter()
                .map(|object| object.bounding_box(tr.clone()))
                .reduce(AABB::surrounding_box)
                .unwrap(),
        }
    }
}
