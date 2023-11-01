use crate::vec::{Point3, Vec3};
use crate::ray::{Ray};
use crate::material::{Scatter};
use crate::hit::{Hit, HitRecord};
use super::aabb::{AABB};
use std::ops::Range;
use std::sync::{Arc};

pub type World = Vec<Arc<Box<dyn Hit>>>;

impl Hit for World {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        let mut temp_record = None;
        let mut closest = time_range.end;

        for object in self {
            if let Some(record) = object.hit(r, time_range.start..closest) {
                closest = record.t;
                temp_record = Some(record);
            }
        }
        temp_record
    }

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        match self.len() {
            0 => AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            1 => self.first().unwrap().bounding_box(time_range),

            // TODO: EXPLAIN
            _ => self
                .iter()
                .map(|object| object.bounding_box(time_range.clone()))
                .reduce(AABB::surrounding_box)
                .unwrap(),
        }
    }
}
