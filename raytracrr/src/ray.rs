use super::vec::{Vec3, Point3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3, 
    pub time: f64
}

impl Ray {
    pub fn new_(origin: Point3, direction: Vec3, t: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time: t
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time: 0.0
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

}
