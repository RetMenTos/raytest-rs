pub mod color;
pub mod hit;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod tri;

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
    // Image dimensions
    let xlen: u32 = 1280;
    let aratio: f64 = 16.0 / 9.0;
    let ylen = (xlen as f64 / aratio).ceil() as u32;

    // Viewport dimensions
    let vylen: f64 = 2.0;
    let vxlen: f64 = vylen * (xlen as f64 / ylen as f64);
    let foclen: f64 = 1.0;

    // World positions
    let camorigin: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    let vwright: Vector3<f64> = Vector3::new(vxlen, 0.0, 0.0);
    let vwdown: Vector3<f64> = Vector3::new(0.0, -vylen, 0.0);

    // Dist between pixels (vert/hor)
    let deltaright = vwright / xlen as f64;
    let deltadown = vwdown / ylen as f64;

    // Viewport world positions
    let vwupleft: Vector3<f64> =
        camorigin - Vector3::new(0.0, 0.0, foclen) - vwright / 2.0 - vwdown / 2.0;
    let pixel0loc: Vector3<f64> = vwupleft + (deltadown + deltaright) * 0.5;

    // Initialize file
    let path = Path::new("testfilers.ppm");
    let display = path.display();
    let mut img_file = match File::create(&path) {
        Err(why) => panic!("File creation failed, {}, {}", display, why),
        Ok(file) => file,
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
    let mut out_str = String::new();
    out_str.push_str(&format!("P3\n{} {}\n255\n", xlen, ylen));
    for y in 0..ylen {
        for x in 0..xlen {
            // For every pixel, get ray, eval color, construct and append rgb value to out
            let pixelloc = pixel0loc + (deltadown * y as f64) + (deltaright * x as f64);
            let mut raydir = pixelloc - camorigin;
            raydir.normalize();
            let currray = Ray {
                origin: camorigin,
                direction: raydir,
            };
            let pixel = construct_pixel(currray.coltrace(&world));
            out_str.push_str(&pixel);
        }
        let completed = y as f64 / ylen as f64 * 100.0;
        print!("\rRendered: {:.2}%", completed);
        let _ = std::io::stdout().flush();
    }

    println!("\nDone!");
    // write to ppm file and open
    match img_file.write_all(out_str.as_bytes()) {
        Err(why) => panic!("File write failed, {}, {}", display, why),
        Ok(_) => println!("Wrote to {}", display),
    };

    // Debug: open in kitten icat
    open_ppm(&display.to_string());
}

fn construct_pixel(c: Vector3<f64>) -> String {
    format!(
        "{} {} {}\n",
        c.get_x() as u32,
        c.get_y() as u32,
        c.get_z() as u32
    )
}

fn open_ppm(file: &String) {
    Command::new("kitten")
        .arg("icat")
        .arg(file)
        .spawn()
        .expect("File open failed");
}
