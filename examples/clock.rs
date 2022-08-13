use core::f64::consts::PI;
use trace::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(500, 500);
    let twelve = Mat4::identity().translate(0, (3.0 / 8.0) * canvas.width as f64, 0)
        * Tuple::from(point![0, 0, 0]);
    let radian = -PI / 6.0;
    for h in 0..12 {
        let hour = Mat4::identity().rotate_z(h as f64 * radian) * twelve;
        let x = (hour.x + canvas.width as f64 / 2.0) as usize;
        let y = (canvas.height as f64 / 2.0 - hour.y) as usize;
        canvas[(x, y)] = color![1, 1, 1];
    }
    canvas.write("clock.ppm")?;
    Ok(())
}
