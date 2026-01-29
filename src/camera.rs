use crate::color;
use crate::hit::HittableList;
use crate::ray::Ray;
use std::io::Write;
use vec3_rs::{self, Vector3};

pub struct Camera {
    pub imagewidth: u32,
    pub aspectratio: f64,
}

impl Camera {
    pub fn render(self, scene: &HittableList) -> String {
        let imageheight = (self.imagewidth as f64 / self.aspectratio).ceil() as u32;

        let vwheight = 2.0;
        let vwlength = vwheight * (self.imagewidth as f64 * imageheight as f64);
        let foclength = 1.0;

        let camorigin: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
        let vwright: Vector3<f64> = Vector3::new(vwlength, 0.0, 0.0);
        let vwdown: Vector3<f64> = Vector3::new(0.0, -vwheight, 0.0);

        let deltaright = vwright / vwlength as f64;
        let deltadown = vwdown / vwheight as f64;

        // Viewport world positions
        let vwupleft: Vector3<f64> =
            camorigin - Vector3::new(0.0, 0.0, foclength) - vwright / 2.0 - vwdown / 2.0;
        let pixel0loc: Vector3<f64> = vwupleft + (deltadown + deltaright) * 0.5;

        let mut out_str = String::new();
        out_str.push_str(&format!("P3\n{} {}\n255\n", self.imagewidth, imageheight));
        for y in 0..imageheight {
            for x in 0..self.imagewidth {
                // For every pixel, get ray, eval color, construct and append rgb value to out
                let pixelloc = pixel0loc + (deltadown * y as f64) + (deltaright * x as f64);
                let mut raydir = pixelloc - camorigin;
                raydir.normalize();
                let currray = Ray {
                    origin: camorigin,
                    direction: raydir,
                };
                let pixel = color::construct_pixel(currray.coltrace(&scene));
                out_str.push_str(&pixel);
            }
            let completed = y as f64 / imageheight as f64 * 100.0;
            print!("\rRendered: {:.2}%", completed);
            let _ = std::io::stdout().flush();
        }
        println!("\nDone!");
        return out_str;
    }
}
