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
    fn to(&self, &t: &f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}

fn main() {
    // Image dimensions
    let xlen: u32 = 256;
    let aratio: f64 = 16.0 / 9.0;
    let ylen = (xlen as f64 / aratio).ceil() as u32;

    // Viewport dimensions
    let vxlen: f64 = 2.0;
    let vylen: f64 = vxlen / (xlen as f64 / ylen as f64);

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
    for y in 0..xlen {
        for x in 0..ylen {
            // TODO: Calculate ray vectors and result color
            // then construct and push pixels
            let pixel = construct_pixel(&x, &y, &0);
            out_str.push_str(&pixel);
        }
    }

    // write to ppm file and open
    match img_file.write_all(out_str.as_bytes()) {
        Err(why) => panic!("File write failed, {}, {}", display, why),
        Ok(_) => println!("Wrote to {}", display),
    };

    open_ppm(&display.to_string());
}

fn construct_pixel(r: &u32, g: &u32, b: &u32) -> String {
    format!("{r} {g} {b}\n")
}

fn open_ppm(file: &String) {
    Command::new("kitten")
        .arg("icat")
        .arg(file)
        .spawn()
        .expect("File open failed");
}
