use super::vec::{Point3, Vec3};
use super::ray::{Ray};
use super::material::{Scatter};
use std::rc::Rc;

pub type World = Vec<Box<dyn Hit>>;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Scatter>,
    pub t: f64,
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

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_record = None;
        let mut closest = t_max;

        for object in self {
            if let Some(record) = object.hit(r, t_min, closest) {
                closest = record.t;
                temp_record = Some(record);
            }
        }
        temp_record
    }
}
