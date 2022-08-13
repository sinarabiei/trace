use trace::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let ray_origin = point![0, 0, -5];
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    // Size of a single pixel in world space units
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let red = color![1, 0, 0];
    let shape = Sphere::new();
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = point![world_x, world_y, wall_z];
            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).normalize(),
            };
            let intersections = shape.intersect(ray);
            match Intersection::hit(&intersections) {
                Some(_) => canvas[(x, y)] = red,
                None => (),
            }
        }
    }
    canvas.write("silhouette.ppm")?;
    Ok(())
}
