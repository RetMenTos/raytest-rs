use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use vec3_rs::{self, Vector3};

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    // Ray to given t
    fn to(&self, &t: &f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }

    // Sphere intersection formula
    fn hitsphere(centre: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
        let oc = centre - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = -2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        // (discriminant >= 0.0) as bool
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }

    // Get color from raytrace
    fn coltrace(&self) -> Vector3<f64> {
        // Hardcoded spheres in world
        let t = Self::hitsphere(Vector3::new(0.0, 0.0, -1.0), 0.5, self);
        if t > 0.0 {
            let nd = self.to(&t) - Vector3::new(0.0, 0.0, 1.0);
            let n = nd / nd.magnitude();
            return Vector3::new(n.get_x() + 1.0, n.get_y() + 1.0, n.get_z() + 1.0) * 255.999 * 1.5;
        }

        // "Sky" gradient
        let normdir = self.direction / self.direction.magnitude();
        let lerpval = 0.5 * (-normdir.get_y() + 1.0);
        Vector3::new(70.0, 100.0, 255.0) * (1.0 - lerpval)
            + Vector3::new(255.0, 255.0, 255.0) * lerpval
    }
}

fn main() {
    // Image dimensions
    let xlen: u32 = 1600;
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

    // Generate string for ppm file
    let mut out_str = String::new();
    out_str.push_str(&format!("P3\n{} {}\n255\n", xlen, ylen));
    for y in 0..ylen {
        for x in 0..xlen {
            // For every pixel, get ray, eval color, construct and append rgb value to out
            let pixelloc = pixel0loc + (deltadown * y as f64) + (deltaright * x as f64);
            let raydir = pixelloc - camorigin;
            let currray = Ray {
                origin: camorigin,
                direction: raydir,
            };
            let pixel = construct_pixel(currray.coltrace());
            out_str.push_str(&pixel);
        }
        let completed = y as f64 / ylen as f64 * 100.0;
        println!("Rendered: {:.2}%", completed)
    }

    println!("Done!");
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
