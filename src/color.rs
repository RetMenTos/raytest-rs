use vec3_rs::{self, Vector3};

pub fn colorize(colorin: Vector3<f64>) -> Vector3<f64> {
    return Vector3::new(
        colorin.get_x() * 255.999,
        colorin.get_y() * 255.999,
        colorin.get_z() * 255.999,
    );
}

pub fn construct_pixel(c: Vector3<f64>) -> String {
    format!(
        "{} {} {}\n",
        c.get_x() as u32,
        c.get_y() as u32,
        c.get_z() as u32
    )
}
