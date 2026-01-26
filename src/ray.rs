use crate::color::colorize;
use crate::hit::{Hittable, HittableList};
use crate::interval::Interval;

use vec3_rs::{self, Vector3};

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    // Ray to given t
    pub fn rayto(&self, &t: &f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }

    // Get color from raytrace
    pub fn coltrace(&self, hittables: &HittableList) -> Vector3<f64> {
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
