use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use vec3_rs::{self, Vector3};

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

struct Sphere {
    centre: Vector3<f64>,
    radius: f64,
}

struct Tri {
    a: Vector3<f64>,
    b: Vector3<f64>,
    c: Vector3<f64>,
}

struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

struct Hit {
    pos: Vector3<f64>,
    normal: Vector3<f64>,
    t: f64,
    frontface: bool,
}

struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    fn size(&self) -> f64 {
        self.max - self.min
    }

    fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, raytminmax: &Interval) -> Option<Hit>;
}

impl Ray {
    // Ray to given t
    fn to(&self, &t: &f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }

    // Get color from raytrace
    fn coltrace(&self, hittables: &HittableList) -> Vector3<f64> {
        // Return color if ray hits something
        match hittables.hit(
            &self,
            &Interval {
                min: 0.0,
                max: f64::INFINITY,
            },
        ) {
            Some(hitrec) => {
                return colorize(hitrec.normal);
            }
            None => {}
        }

        // Otherwise return "sky" gradient
        let normdir = self.direction / self.direction.magnitude();
        let lerpval = 0.5 * (-normdir.get_y() + 1.0);
        return Vector3::new(70.0, 100.0, 255.0) * (1.0 - lerpval)
            + Vector3::new(255.0, 255.0, 255.0) * lerpval;
    }
}

impl Hit {
    fn setfacenormal(&mut self, ray: &Ray, outnorm: Vector3<f64>) {
        // Outward normal vector should be normalized
        self.frontface = ray.direction.dot(&outnorm) < 0.0;
        self.normal = if self.frontface {
            outnorm
        } else {
            outnorm * -1.0
        };
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, raytminmax: &Interval) -> Option<Hit> {
        let mut temprec: Option<Hit> = None;
        let mut closest: f64 = raytminmax.max;

        for object in &self.hittables {
            match object.hit(
                &ray,
                &Interval {
                    min: raytminmax.min.clone(),
                    max: closest.clone(),
                },
            ) {
                Some(hitrec) => {
                    closest = hitrec.t;
                    temprec = Some(hitrec);
                }
                None => {}
            };
        }

        return temprec;
    }
}

impl Hittable for Sphere {
    // Calculate if given ray hit self, provide hitrec
    fn hit(&self, ray: &Ray, raytminmax: &Interval) -> Option<Hit> {
        let oc = self.centre - ray.origin;
        let a = ray.direction.magnitude() * ray.direction.magnitude();
        let h = ray.direction.dot(&oc);
        let c = oc.magnitude() * oc.magnitude() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !raytminmax.contains(root) {
            root = (h + sqrtd) / a;
            if !raytminmax.contains(root) {
                return None;
            }
        }

        let pos = ray.to(&root);
        let normal = (pos - self.centre) / self.radius;
        let mut hitrec = Hit {
            t: root,
            pos,
            normal,
            frontface: false,
        };
        hitrec.setfacenormal(ray, normal);
        return Some(hitrec);
    }
}

impl Hittable for Tri {
    fn hit(&self, ray: &Ray, raytminmax: &Interval) -> Option<Hit> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let normal = e1.cross(&e2);
        let det = ray.direction.dot(&normal) * -1.0;

        if det != 0.0 {
            let invdet = 1.0 / det;
            let ao = ray.origin - self.a;
            let dao = ao.cross(&ray.direction);
            let u = invdet * e2.dot(&dao);
            let v = e1.dot(&dao) * -1.0 * invdet;
            let t = normal.dot(&ao) * invdet;
            if !raytminmax.contains(t) {
                return None;
            }

            if det < 0.0 && u > 0.0 && v > 0.0 && t > 0.0 && (u + v) < 1.0 {
                let pos = ray.origin + ray.direction * t;
                let mut hitrec = Hit {
                    pos,
                    normal,
                    t,
                    frontface: false,
                };
                hitrec.setfacenormal(ray, normal);
                return Some(hitrec);
            }
        }
        return None;
    }
}

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

fn colorize(colorin: Vector3<f64>) -> Vector3<f64> {
    return Vector3::new(
        colorin.get_x() * 255.999,
        colorin.get_y() * 255.999,
        colorin.get_z() * 255.999,
    );
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
