mod vec;
mod ray;
mod hit;
mod camera;
mod material;
mod texture;
mod perlin;

use std::io::{stderr, Write};
use hit::block::Block;
use hit::constant_medium::ConstantMedium;
use hit::rect::Rect;
use hit::rotate::Rotate;
use hit::translate::Translate;
use material::diffuse::Diffuse;
use rand::distributions::Uniform;
use rand::{Rng, thread_rng};
use std::sync::Arc;

use vec::{Vec3, Colour, Point3};
use ray::{Ray};
use hit::{Hit};
use hit::world::{World};
use hit::sphere::{Sphere};
use hit::moving_sphere::{MovingSphere};
use camera::{Camera};
use crate::hit::bvh::BVH;
use crate::hit::quad::Quad;
use crate::material::{matte::Matte, metal::Metal, dielectric::Dielectric};
use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::texture::solid::Solid;

fn ray_colour(r: &Ray, background: Colour, world: &World, depth: u64) -> Colour {
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
        let emit = record.material.as_ref().emitted(record.u, record.v, record.p);
        if let Some((attenuation, scattered)) = record.material.scatter(r, &record) {
            emit + attenuation * ray_colour(&scattered, background, world, depth - 1)
        } else {
            emit
        }
    } else {
        // let unit_direction = r.direction().normalized();
        // let t = 0.5 * (unit_direction.y() as f64 + 1.0);
        // (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
        background
    }
}

fn cornell_box() -> World {

    let mut world = World::new();

    let red = Matte::new_arc(Solid::new_arc(Colour::new(0.65, 0.05, 0.05)));
    let white = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.73, 0.73, 0.73))));
    let green = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.12, 0.45, 0.15))));
    let light = Diffuse::new_arc(Arc::new(Solid::new(Colour::new(15.0, 15.0, 15.0))));
    
    let left = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 2, green);
    let right = Rect::new(0.0..555.0, 0.0..555.0, 0.0, 2, red);
    let lightRect = Rect::new(213.0..343.0, 113.0..332.0, 554.0, 1, light.clone());
    let bottom = Rect::new(0.0..555.0, 0.0..555.0, 0.0, 1, white.clone());
    let top = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 1, white.clone());
    let back = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 0, white.clone());

    let box1 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone()
    );
    let mut box1_rotate = Rotate::new(Arc::new(box1), 35.0, 1);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), 25.0, 0);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), -15.0, 2);
    let box1_translate = Translate::new(Arc::new(box1_rotate), Vec3::new(265.0, 0.0, 295.0));

    let box2 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2_rotate = Rotate::new(Arc::new(box2), -18.0, 1);
    let box2_translate = Translate::new(Arc::new(box2_rotate), Vec3::new(130.0, 0.0, 65.0));
        
    world.push(Arc::new(Box::new(left)));
    world.push(Arc::new(Box::new(right)));
    world.push(Arc::new(Box::new(bottom)));
    world.push(Arc::new(Box::new(top)));
    world.push(Arc::new(Box::new(back)));
    world.push(Arc::new(Box::new(lightRect)));
    world.push(Arc::new(Box::new(box1_translate)));
    world.push(Arc::new(Box::new(box2_translate)));

    world
}

fn cornell_smoke() -> World {

    let mut world = World::new();

    let red = Matte::new_arc(Solid::new_arc(Colour::new(0.65, 0.05, 0.05)));
    let white = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.73, 0.73, 0.73))));
    let green = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.12, 0.45, 0.15))));
    let light = Diffuse::new_arc(Arc::new(Solid::new(Colour::new(15.0, 15.0, 15.0))));
    
    let left = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 2, green);
    let right = Rect::new(0.0..555.0, 0.0..555.0, 0.0, 2, red);
    let lightRect = Rect::new(213.0..343.0, 113.0..332.0, 554.0, 1, light.clone());
    let bottom = Rect::new(0.0..555.0, 0.0..555.0, 0.0, 1, white.clone());
    let top = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 1, white.clone());
    let back = Rect::new(0.0..555.0, 0.0..555.0, 555.0, 0, white.clone());

    let box1 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone()
    );
    let mut box1_rotate = Rotate::new(Arc::new(box1), 35.0, 1);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), 25.0, 0);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), -15.0, 2);
    let box1_translate = Translate::new_arc(Arc::new(box1_rotate), Vec3::new(265.0, 0.0, 295.0));

    let box2 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2_rotate = Rotate::new(Arc::new(box2), -18.0, 1);
    let box2_translate = Translate::new_arc(Arc::new(box2_rotate), Vec3::new(130.0, 0.0, 65.0));
        
    world.push(Arc::new(Box::new(left)));
    world.push(Arc::new(Box::new(right)));
    world.push(Arc::new(Box::new(bottom)));
    world.push(Arc::new(Box::new(top)));
    world.push(Arc::new(Box::new(back)));
    world.push(Arc::new(Box::new(lightRect)));
    world.push(ConstantMedium::new_arc(box1_translate, Solid::new_arc(Colour::new(1.0, 1.0, 1.0)), 0.06));
    world.push(ConstantMedium::new_arc(box2_translate, Solid::new_arc(Colour::new(0.0, 1.0, 0.0)), 0.05));

    world
}

fn cornell_smoke_without_light() -> World {

    let mut world = World::new();

    let white = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.73, 0.73, 0.73))));
    let green = Matte::new_arc(Arc::new(Solid::new(Colour::new(0.12, 0.45, 0.15))));

    let box1 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone()
    );
    let mut box1_rotate = Rotate::new(Arc::new(box1), 35.0, 1);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), 25.0, 0);
    box1_rotate = Rotate::new(Arc::new(box1_rotate), -15.0, 2);
    let box1_translate = Translate::new_arc(Arc::new(box1_rotate), Vec3::new(265.0, 0.0, 295.0));

    let box2 = Block::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2_rotate = Rotate::new(Arc::new(box2), -18.0, 1);
    let box2_translate = Translate::new_arc(Arc::new(box2_rotate), Vec3::new(130.0, 0.0, 65.0));
        
    let checker = Checker::new_texture_arc(0.32, Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9)
    );
    let mat_ground = Matte::new_arc(checker);
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground.clone());

    world.push(ConstantMedium::new_arc(box1_translate, Solid::new_arc(Colour::new(1.0, 1.0, 1.0)), 0.06));
    world.push(ConstantMedium::new_arc(box2_translate, Solid::new_arc(Colour::new(0.0, 1.0, 0.0)), 0.01));
    world.push(Arc::new(Box::new(ground_sphere)));

    world
}


fn simple_light() -> World {
    let mut world = World::new();

    let mat_perlin = Matte::new_arc(Noise::new_arc(4.0));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_perlin.clone());
    let sphere_center = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat_perlin);

    let difflight = Diffuse::new_arc(Solid::new_arc(Colour::new(4.0,4.0,4.0)));
    let _light = Quad::new(
        Point3::new(3.0,1.0,-1.0), 
        Vec3::new(-7.0,0.0,0.0), 
        Vec3::new(0.0,-2.0,0.0), 
        difflight.clone()
    );
    let light = Rect::new(3.0..5.0, 1.0..3.0, -1.0, 0, difflight.clone());
    let lightball = Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight);

    world.push(Arc::new(Box::new(ground_sphere)));
    world.push(Arc::new(Box::new(light)));
    // world.push(Arc::new(Box::new(lightball)));
    world.push(Arc::new(Box::new(sphere_center)));

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
                let sphere_mat = Metal::new_arc(Solid::new_arc(albedo), fuzz);
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = MovingSphere::new(
                    center, 
                    center1,
                    0.2, 
                    0.0..1.0,
                    sphere_mat);
                world.push(Arc::new(Box::new(sphere)));
            } else {
                // Glass
                let sphere_mat = Dielectric::new_arc(1.5);
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = MovingSphere::new(
                    center, 
                    center1,
                    0.2, 
                    0.0..1.0,
                    sphere_mat);
                world.push(Arc::new(Box::new(sphere)));
            }
        }
    }

    let checker = Checker::new_texture_arc(0.32, 
            Colour::new(0.2, 0.3, 0.1),
            Colour::new(0.9, 0.9, 0.9)
    );
    let mat_perlin = Matte::new_arc(Noise::new_arc(4.0));
    let mat_ground = Matte::new_arc(checker);
    let mat_center = Matte::new_arc(Arc::new(Image::new("earth.jpg").unwrap()));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Metal::new_arc(Solid::new_arc(Colour::new(0.8, 0.6, 0.2)), 0.0);

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

pub fn final_scene() -> World {
    let mut world = World::new();
    let mut rand_eng = thread_rng();

    let ground = Matte::new_arc(Solid::new_arc(Colour::new(0.48, 0.83, 0.53)));

    let rand_dst = Uniform::from(1.0..=100.0);
    const BOXES_PER_SIDE: u32 = 20;

    let block = Block::new_arc(
        Vec3::new(-1000.0, 0.0, -1000.0),
        Vec3::new(1000.0, 0.0, 1000.0),
        ground.clone(),
    );
    world.push(block);
    
    // for i in 0..BOXES_PER_SIDE {
    //     let i = f64::from(i);
    //     for j in 0..BOXES_PER_SIDE {
    //         let j = f64::from(j);
    //         const W: f64 = 100.0;
    //         let x0 = -1000.0 + i * W;
    //         let z0 = -1000.0 + j * W;
    //         let y0 = 0.0;
    //         let x1 = x0 + W;
    //         let y1 = rand_eng.sample(rand_dst);
    //         let z1 = z0 + W;

    //         let block = Block::new_arc(
    //             Vec3::new(x0, y0, z0),
    //             Vec3::new(x1, y1, z1),
    //             ground.clone(),
    //         );
    //         world.push(block);
    //     }
    // }

    let light = Diffuse::new_arc(Solid::new_arc(Colour::new(7.0, 7.0, 7.0)));
    world.push(Rect::new_arc(
        123.0..423.0,
        147.0..412.0,
        554.0,
        1,
        light
    ));

    let centre1 = Vec3::new(400.0, 400.0, 200.0);
    let centre2 = centre1 + Vec3::new(29.0, 0.0, 0.0);
    let moving_sphere_material = Matte::new_arc(Solid::new_arc(Colour::new(0.7, 0.3, 0.1)));
    world.push(MovingSphere::new_arc(
        centre1,
        centre2,
        50.0,
        0.0..1.0,
        moving_sphere_material,
    ));

    // // glass
    world.push(Arc::new(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new_arc(1.5),
    ))));

    // matte lower right
    world.push(Arc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new_arc(Solid::new_arc(Colour::new(0.8, 0.8, 0.9)), 1.0),
    ))));

    // 
    let boundary = Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        71.0,
        Dielectric::new_arc(1.5),
    );
    world.push(Arc::new(Box::new(boundary)));
    let boundary = Sphere::new_arc(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new_arc(1.5),
    );
    world.push(ConstantMedium::new_arc(
        boundary,
        Solid::new_arc(Colour::new(0.2, 0.4, 0.9)),
        0.2,
    ));

    let boundary = Sphere::new_arc(
        Vec3::new(0.0, 0.0, 0.0), 
        5000.0, 
        Dielectric::new_arc(1.5)
    );
    world.push(ConstantMedium::new_arc(
        boundary,
        Solid::new_arc(Colour::new(1.0, 1.0, 1.0)),
        0.0001
    ));

    let emat = Matte::new_arc(Arc::new(Image::new("earth.jpg").unwrap()));
    world.push(Arc::new(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0), 
        100.0, 
        emat)))
    );

    let pertext = Noise::new_arc(0.1);
    world.push(Arc::new(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Matte::new_arc(pertext),
    ))));

    // let mut boxes2: Vec<_> = Vec::new();
    // let white = Matte::new_arc(Solid::new_arc(Colour::new(0.73, 0.73, 0.73)));
    // const NS: u32 = 15;
    // for _ in 0..NS {
    //     boxes2.push(Sphere::new_hittable(
    //         Vec3::random(0.0..165.0),
    //         10.0,
    //         white.clone(),
    //     ));
    // }

    // world.push(Arc::new(Box::new(Translate::new(
    //     Rotate::new_arc(Arc::new(BVH::new(&mut boxes2, 0.0..1.0)), 15.0, 1),
    //     Vec3::new(-100.0, 270.0, 395.0),
    // ))));

    world
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u64 = 800; // 800
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 1000; // 10000
    const MAX_DEPTH: u64 = 10; // 50

    // WORLD
    let world = final_scene();

    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(lookfrom,
        lookat,
        vup,
        40.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        Colour::new(0.00, 0.00, 0.00)
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
                pixel += ray_colour(&r, camera.background, &world, MAX_DEPTH);
            }

            println!("{}", pixel.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
