use crate::hit::HittableList;
use vec3_rs::{self, Vector3};

pub struct Camera {
    pub imagewidth: u32,
    pub aspectratio: f64,
    vwheight: f64,
    vwlength: f64,
    foclength: Vector3<f64>,
    camorigin: Vector3<f64>,
}

impl Camera {
    fn render(self, scene: &HittableList) {
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
    }
}
