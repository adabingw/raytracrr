mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::io::{stderr, Write};
use rand::Rng;
use vec::{Vec3, Colour, Point3};
use ray::{Ray};
use hit::{Hit, World};
use sphere::{Sphere};
use camera::{Camera};
use material::{Scatter};

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

    if let Some(record) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = record.material.scatter(r, &record) {
            attenuation.cross(ray_colour(&scattered, world, depth - 1))
        } else {
            Colour::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() as f64 + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // WORLD
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.push(Box::new(Sphere::new(Point3::new(0.7, -0.25, -1.0), 0.3)));

    let camera = Camera::new();

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
