use crate::Interval;
use crate::Ray;
use vec3_rs::{self, Vector3};

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

pub struct Hit {
    pub pos: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub frontface: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, raytminmax: &Interval) -> Option<Hit>;
}

impl Hit {
    pub fn setfacenormal(&mut self, ray: &Ray, outnorm: Vector3<f64>) {
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
