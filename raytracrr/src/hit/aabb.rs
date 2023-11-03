use std::{ops::Range, mem, cmp::Ordering};

use crate::{vec::Vec3, ray::Ray};

// minimum.x, maximum.x represents the x range
// minimum.y, maximum.y represents the y range, etc
// when given 2 bounding boxes, the minimum.x is the min(minimum1.x, minimum2.x), etc
#[derive(Clone)]
pub struct AABB {
    minimum: Vec3, 
    maximum: Vec3
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB {
            minimum, maximum
        }
    }

    pub fn surrounding_box(bbox1: AABB, bbox2: AABB) -> AABB {
        let Vec3 { e: [bb1minx, bb1miny, bb1minz] } = bbox1.minimum;
        let Vec3 { e: [bb1maxx, bb1maxy, bb1maxz] } = bbox1.maximum;
        let Vec3 { e: [bb2minx, bb2miny, bb2minz] } = bbox2.minimum;
        let Vec3 { e: [bb2maxx, bb2maxy, bb2maxz] } = bbox2.maximum;

        let minimum = Vec3::new(bb1minx.min(bb2minx), bb1miny.min(bb2miny), bb1minz.min(bb2minz));
        let maximum = Vec3::new(bb1maxx.min(bb2maxx), bb1maxy.min(bb2maxy), bb1maxz.min(bb2maxz));

        AABB::new(minimum, maximum)
    }

    pub fn get_minimum(&self) -> Vec3 {
        self.minimum
    }

    pub fn get_maximum(&self) -> Vec3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, time_range: Range<f64>) -> bool {
        let mut tmin = time_range.start;
        let mut tmax = time_range.end;

        let Vec3 { e: [minx, miny, minz] } = self.minimum;
        let Vec3 { e: [maxx, maxy, maxz] } = self.maximum;
        let Vec3 { e: [ox, oy, oz] } = r.orig;
        let Vec3 { e: [dx, dy, dz] } = r.dir;

        hit_1d(minx, maxx, ox, dx, &mut tmin, &mut tmax) &&
        hit_1d(miny, maxy, oy, dy, &mut tmin, &mut tmax) &&
        hit_1d(minz, maxz, oz, dz, &mut tmin, &mut tmax)
    }
}

pub fn hit_1d(minimum: f64, maximum: f64, origin: f64, direction: f64, tmin: &mut f64, tmax: &mut f64) -> bool {
    let mut t0 = (minimum - origin) / direction;
    let mut t1 = (maximum - origin) / direction;

    if direction < 0.0 {
        mem::swap(&mut t0, &mut t1);
    }

    *tmin = t0.max(*tmin);
    *tmax = t1.max(*tmax);

    tmin < tmax
}
