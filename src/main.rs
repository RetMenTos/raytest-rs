pub mod camera;
pub mod color;
pub mod hit;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod tri;

use crate::camera::Camera;
use crate::hit::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tri::Tri;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use vec3_rs::{self, Vector3};

fn main() {
    // Initialize file
    let path = Path::new("testfilers.ppm");
    let display = path.display();
    let mut img_file = match File::create(&path) {
        Err(why) => panic!("File creation failed, {}, {}", display, why),
        Ok(file) => file,
    };

    let camera = Camera {
        imagewidth: 1600,
        aspectratio: 16.0 / 9.0,
    };

    // World objects
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.hittables.push(Box::new(Sphere {
        centre: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.hittables.push(Box::new(Sphere {
        centre: Vector3::new(2.0, 2.0, -2.0),
        radius: 1.0,
    }));
    world.hittables.push(Box::new(Sphere {
        centre: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));
    world.hittables.push(Box::new(Tri {
        a: Vector3::new(-3.0, 2.0, -2.0),
        b: Vector3::new(0.0, 1.0, -2.0),
        c: Vector3::new(0.0, 0.0, -0.2),
    }));
    world.hittables.push(Box::new(Tri {
        a: Vector3::new(0.0, 1.0, -2.0),
        b: Vector3::new(3.0, 2.0, -2.0),
        c: Vector3::new(0.0, 0.0, -0.2),
    }));

    // Generate string for ppm file
    // write to ppm file and open
    match img_file.write_all(camera.render(&world).as_bytes()) {
        Err(why) => panic!("File write failed, {}, {}", display, why),
        Ok(_) => println!("Wrote to {}", display),
    };

    // Debug: open in kitten icat
    open_ppm(&display.to_string());
}

fn open_ppm(file: &String) {
    Command::new("kitten")
        .arg("icat")
        .arg(file)
        .spawn()
        .expect("File open failed");
}
