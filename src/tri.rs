use crate::hit::Hit;
use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use vec3_rs::{self, Vector3};

pub struct Tri {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
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
