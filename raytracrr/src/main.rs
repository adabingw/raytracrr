mod vec;
mod ray;
mod hit;
mod camera;
mod material;
mod texture;
mod perlin;

use std::io::{stderr, Write};
use rand::Rng;
use std::sync::Arc;

use vec::{Vec3, Colour, Point3};
use ray::{Ray};
use hit::{Hit};
use hit::world::{World};
use hit::sphere::{Sphere};
use hit::moving_sphere::{MovingSphere};
use camera::{Camera};
use crate::hit::quad::Quad;
use crate::material::{matte::Matte, metal::Metal, dielectric::Dielectric};
use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::texture::solid::Solid;

fn ray_colour(r: &Ray, world: &World, depth: u64) -> Colour {
    // ray going from origin (camera eye) to point on the screen
    // linearly blends white and blue depending on the height of the y coordinate 
    // after scaling the ray direction to unit length (−1.0 < y < 1.0). 

    // because looking at the y height after normalizing the vector, 
    // there's a horizontal gradient in addition to the vertical gradient.

    // standard graphics trick of scaling that to 0.0 ≤ t ≤ 1.0
    // t = 1.0 -> blue, t = 0.0 -> white, in between -> blend 
    // forms "linear interpolation" between two things. A lerp is always of the form
    // blendedValue = (1 − t) ⋅ startValue + t ⋅ endValue, 0.0 ≤ t ≤ 1.0
    if depth <= 0 {
        // exceeded ray bounce limit, no light gathered
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(r, 0.001..f64::INFINITY) {
        if let Some((attenuation, scattered)) = record.material.scatter(r, &record) {
            attenuation * ray_colour(&scattered, world, depth - 1)
        } else {
            Colour::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() as f64 + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}

fn quads() -> World {
    let mut world = World::new();

    let left_red = Arc::new(Matte::new(Arc::new(Solid::new(Colour::new(1.0, 0.2, 0.2)))));
    let back_green = Arc::new(Matte::new(Arc::new(Solid::new(Colour::new(0.2, 0.2, 1.0)))));
    let right_blue = Arc::new(Matte::new(Arc::new(Solid::new(Colour::new(0.7, 0.2, 0.2)))));
    let upper_orange = Arc::new(Matte::new(Arc::new(Solid::new(Colour::new(1.0, 0.3, 0.4)))));
    let lower_teal = Arc::new(Matte::new(Arc::new(Solid::new(Colour::new(0.2, 0.8, 0.8)))));

    let left_quad = Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red);
    let back_quad = Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green);
    let right_quad = Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue);
    let upper_quad = Quad::new(Point3::new(-2.0, 2.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange);
    let lower_quad = Quad::new(Point3::new(-2.0, -2.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal);

    // Quads
    world.push(Arc::new(Box::new(left_quad)));
    world.push(Arc::new(Box::new(back_quad)));
    world.push(Arc::new(Box::new(right_quad)));
    world.push(Arc::new(Box::new(upper_quad)));
    world.push(Arc::new(Box::new(lower_quad)));
    world
}

fn lots_of_spheres() -> World {
    let mut world = World::new();
    let mut rng = rand::thread_rng();

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Colour::random(0.0..1.0) * Colour::random(0.0..1.0);
                let sphere_mat = Arc::new(
                    Matte::new(
                        Arc::new(
                            Solid::new(albedo)
                        )
                    )
                );
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.3), 0.0);
                let sphere = Sphere::new(
                    center, 0.2, sphere_mat
                );
                // let sphere = MovingSphere::new(
                //     center, 
                //     center1,
                //     0.2, 
                //     sphere_mat);
                world.push(Arc::new(Box::new(sphere)));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Colour::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(
                    Metal::new(
                        Arc::new(
                            Solid::new(albedo)
                        ), fuzz
                    )
                );
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = MovingSphere::new(
                    center, 
                    center1,
                    0.2, 
                    sphere_mat);
                world.push(Arc::new(Box::new(sphere)));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = MovingSphere::new(
                    center, 
                    center1,
                    0.2, 
                    sphere_mat);
                world.push(Arc::new(Box::new(sphere)));
            }
        }
    }

    let checker = Arc::new(
        Checker::new_texture(0.32, 
            Colour::new(0.2, 0.3, 0.1),
            Colour::new(0.9, 0.9, 0.9)
        )
    );
    let mat_perlin = Arc::new(Matte::new(Arc::new(Noise::new(4.0))));
    let mat_ground = Arc::new(Matte::new(checker));
    let mat_center = Arc::new(
        Matte::new(
            Arc::new(
                Image::new("earth.jpg").unwrap()
            )
        )
    );
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Arc::new(Solid::new(Colour::new(0.8, 0.6, 0.2))), 0.0));

    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_perlin.clone());
    let sphere_center = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat_center);
    let sphere_left = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat_left);
    let sphere_right = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat_perlin);

    let earth_texture = Image::new("earth.jpg").unwrap();
    let earth_surface: Matte = Matte::new(Arc::new(earth_texture));
    let globe = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Arc::new(earth_surface));

    world.push(Arc::new(Box::new(globe)));
    world.push(Arc::new(Box::new(ground_sphere)));
    world.push(Arc::new(Box::new(sphere_center)));
    world.push(Arc::new(Box::new(sphere_left)));
    world.push(Arc::new(Box::new(sphere_right)));

    world
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 80;
    const MAX_DEPTH: u64 = 5;

    // WORLD
    let mut world = quads();

    let lookfrom = Point3::new(-2.0, -2.0, 8.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(lookfrom,
        lookat,
        vup,
        80.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus
    );

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        // traverse the screen from the upper left hand corner 
        // use two offset vectors along the screen sides to move the ray endpoint across the screen.
        for i in 0..IMAGE_WIDTH {
            let mut pixel = Colour::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u : f64 = rng.gen();
                let random_v : f64 = rng.gen();

                let u = (i as f64 + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + random_v) / ((IMAGE_HEIGHT - 1) as f64);
    
                let r = camera.get_ray(u, v);
                pixel += ray_colour(&r, &world, MAX_DEPTH);
            }

            println!("{}", pixel.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
