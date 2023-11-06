use std::{sync::Arc, f64::INFINITY, ops::Range};

use crate::{vec::Vec3, ray::Ray};

use super::{aabb::AABB, Hit, hit_record::HitRecord};

pub struct Rotate {
    object: Arc<dyn Hit>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
    axis: i32 // 0: x, 1: y, 2: z
}

impl Rotate {
    pub fn new(object: Arc<dyn Hit>, theta: f64, axis: i32) -> Rotate {
        let (sin_theta, cos_theta) = theta.to_radians().sin_cos();
        let bbox = object.bounding_box(0.0..1.0);

        let b_min = bbox.get_minimum();
        let b_max = bbox.get_maximum();

        let mut minx = INFINITY;
        let mut miny = INFINITY;
        let mut minz = INFINITY;
        let mut maxx = -INFINITY;
        let mut maxy = -INFINITY;
        let mut maxz = -INFINITY;

        for i in 0..2 {
            let i = f64::from(i);
            for j in 0..2 {
                let j = f64::from(j);
                for k in 0..2 {
                    let k = f64::from(k);

                    let x = i * b_max.x() + (1.0 - i) * b_min.x();
                    let y = j * b_max.y() + (1.0 - j) * b_min.y();
                    let z = k * b_max.z() + (1.0 - k) * b_min.z();

                    let newx = if axis == 0 {
                        x
                    } else if axis == 1 {
                        cos_theta * x + sin_theta * z
                    } else {
                        cos_theta * x + sin_theta * y
                    };

                    let newy = if axis == 0 {
                        cos_theta * y + sin_theta * z
                    } else if axis == 1 {
                        y
                    } else {
                        -sin_theta * x + cos_theta * y
                    };

                    let newz = if axis == 0 {
                        -sin_theta * y + cos_theta * z
                    } else if axis == 1 {
                        -sin_theta * x + cos_theta * z
                    } else {
                        z
                    };

                    minx = minx.min(newx);
                    maxx = maxx.max(newx);
                    miny = miny.min(newy);
                    maxy = maxy.max(newy);
                    minz = minz.min(newz);
                    maxz = maxz.max(newz);
                }
            }
        }

        let minimum = Vec3::new(minx, miny, minz);
        let maximum = Vec3::new(maxx, maxy, maxz);
        let bounding_box = AABB::new(minimum, maximum);

        Rotate {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
            axis
        }
    }
}

impl Hit for Rotate {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        let Vec3 {e: [ox, oy, oz] } = r.origin();
        let Vec3 {e: [dx, dy, dz] } = r.direction();

        let (o2x, d2x) = if self.axis == 0 {
            (ox, dx)
        } else if self.axis == 1 {
            (self.cos_theta * ox - self.sin_theta * oz, self.cos_theta * dx - self.sin_theta * dz)
        } else {
            (self.cos_theta * ox - self.sin_theta * oy, self.cos_theta * dx - self.sin_theta * dy)
        };

        let (o2y, d2y) = if self.axis == 0 {
            (self.cos_theta * oy - self.sin_theta * oz, self.cos_theta * dy - self.sin_theta * dz)
        } else if self.axis == 1 {
            (oy, dy)
        } else {
            (self.sin_theta * ox + self.cos_theta * oy, self.sin_theta * dx + self.cos_theta * dy)
        };

        let (o2z, d2z) = if self.axis == 0 {
            (self.sin_theta * oy + self.cos_theta * oz, self.sin_theta * dy + self.cos_theta * dz)
        } else if self.axis == 1 {
            (self.sin_theta * ox + self.cos_theta * oz, self.sin_theta * dx + self.cos_theta * dz)
        } else {
            (oz, dz)
        };

        let origin = Vec3::new(o2x, o2y, o2z);
        let direction = Vec3::new(d2x, d2y, d2z);

        let rotated_r = Ray::new(origin, direction);

        self.object.hit(&rotated_r, time_range).map(|rec| {
            let Vec3 { e: [px, py, pz] } = rec.p;
            let Vec3 { e: [nx, ny, nz] } = rec.normal;

            let (p2x, n2x) = if self.axis == 0 {
                (px, nx)
            } else if self.axis == 1 {
                (self.cos_theta * px + self.sin_theta * pz, self.cos_theta * nx + self.sin_theta * nz)
            } else {
                (self.cos_theta * px + self.sin_theta * py, self.cos_theta * nx + self.sin_theta * ny)
            };
    
            let (p2y, n2y) = if self.axis == 0 {
                (self.cos_theta * py + self.sin_theta * pz, self.cos_theta * ny + self.sin_theta * nz)
            } else if self.axis == 1 {
                (py, ny)
            } else {
                (-self.sin_theta * px + self.cos_theta * py, -self.sin_theta * dx + self.cos_theta * dy)
            };
    
            let (p2z, n2z) = if self.axis == 0 {
                (-self.sin_theta * py + self.cos_theta * pz, -self.sin_theta * ny + self.cos_theta * nz)
            } else if self.axis == 1 {
                (-self.sin_theta * px + self.cos_theta * pz, -self.sin_theta * nx + self.cos_theta * nz)
            } else {
                (pz, nz)
            };

            let p = Vec3::new(p2x, p2y, p2z);
            let normal = Vec3::new(n2x, n2y, n2z);

            HitRecord {
                p: p,
                normal: normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                material: rec.material,
                front_face: rec.front_face
            }
        })
    }

    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        self.bounding_box.clone()
    }
}
