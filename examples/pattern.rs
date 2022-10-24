use std::f64::consts::PI;
use trace::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let wall = Plane::new()
        .set_transform(Mat4::identity().rotate_x(-PI / 2.0).rotate_z(PI / 8.0))
        .set_pattern(Box::new(
            Stripe::new(color![0.5, 0.5, 0.5], color![0.15, 0.15, 0.15]).set_transform(
                Mat4::identity()
                    // Width of stripe computed according to
                    // the width of checkers, they are not same
                    // because stripe is rotated.
                    .scale(0.923, 0.923, 0.923)
                    .scale(1.7, 1.7, 1.7),
            ),
        ));

    let floor = Plane::new()
        .set_transform(Mat4::identity())
        .set_pattern(Box::new(
            Checkers::new(color![0.15, 0.15, 0.15], color![0.5, 0.5, 0.5])
                .set_transform(Mat4::identity().scale(1.7, 1.7, 1.7)),
        ));

    let left = Sphere::new()
        .set_transform(
            Mat4::identity()
                .rotate_z(PI / 2.8)
                .rotate_y(-PI / 2.5)
                .scale(2, 2, 2)
                .translate(-2.5, 2, -8),
        )
        .set_pattern(Box::new(
            Ring::new(color![0.35, 0.70, 0.39], color![0.23, 0.41, 0.29])
                .set_transform(Mat4::identity().scale(0.2, 0.2, 0.2)),
        ));

    let right = Sphere::new()
        .set_transform(Mat4::identity().rotate_z(PI / 6.0).translate(1.5, 1, -10))
        .set_pattern(Box::new(
            Gradient::new(color![0.85, 0.16, 0.01], color![0.78, 0.70, 0.27])
                .set_transform(Mat4::identity().scale(2, 2, 2).translate(1, 0, 0)),
        ));

    // World
    let mut world = World::new(Light {
        position: point![-16, 20, -20],
        intensity: color![1, 1, 1],
    });
    world.push(wall);
    world.push(floor);
    world.push(left);
    world.push(right);

    // Camera
    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform =
        Mat4::identity().view_transform(point![-1, 5, -19], point![-1, 0, 0], vector![0, 1, 0]);

    // Canvas
    let canvas = camera.render(&world);

    canvas.write("pattern.ppm")?;
    Ok(())
}
