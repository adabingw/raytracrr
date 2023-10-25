use super::vec::{Point3, Vec3};
use super::ray::{Ray};
use rand::Rng;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3, 
    vertical: Vec3, 
    cu: Vec3, 
    cv: Vec3, 
    lens_radius: f64
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3, 
        vup: Vec3,
        vfov: f64, 
        aspect_ratio: f64,
        aperture: f64, 
        focus_dist: f64
    ) -> Camera {
        // CAMERA
        // viewport's aspect ratio should be the same as rendered image. 
        // pick a viewport two units in height. 
        // We'll also set the distance between the projection plane and the projection point to be one unit.
        
        // vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        // y coordinate points up
        // x coordinate points left
        // z coordinate points towards the plane
        let horizontal = focus_dist * viewport_width * cu;
        let vertical = focus_dist * viewport_height * cv;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_dist * cw;

        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            cu: cu, 
            cv: cv, 
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_sphere();
        let offset = self.cu * rd.x() + self.cv * rd.y();

        let mut rng = rand::thread_rng();
        let random_time : f64 = rng.gen();

        Ray::new_(self.origin + offset,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
                random_time)
    }
}
