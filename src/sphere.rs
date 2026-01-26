use crate::Interval;
use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use vec3_rs::{self, Vector3};

pub struct Sphere {
    pub centre: Vector3<f64>,
    pub radius: f64,
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

        let pos = ray.rayto(&root);
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
