use std::f64::consts::PI;
use trace::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // Floor
    let mut floor = Sphere::new();
    floor.transform = Mat4::identity().scale(10, 0.01, 10);
    floor.material = Material::new();
    floor.material.color = color![1, 0.9, 0.9];
    floor.material.specular = 0.0;

    // Left wall
    let mut left_wall = Sphere::new();
    left_wall.transform = Mat4::identity()
        .scale(10, 0.01, 10)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0, 0, 5);
    left_wall.material = floor.material;

    // Right wall
    let mut right_wall = Sphere::new();
    right_wall.transform = Mat4::identity()
        .scale(10, 0.01, 10)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0, 0, 5);
    right_wall.material = floor.material;

    // Middle sphere
    let mut middle = Sphere::new();
    middle.transform = Mat4::identity().translate(-0.5, 1, 0.5);
    middle.material = Material::new();
    middle.material.color = color![0.1, 1, 0.5];
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // Right sphere
    let mut right = Sphere::new();
    right.transform = Mat4::identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5);
    right.material = Material::new();
    right.material.color = color![0.5, 1, 0.1];
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // Left sphere
    let mut left = Sphere::new();
    left.transform = Mat4::identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75);
    left.material = Material::new();
    left.material.color = color![1, 0.8, 0.1];
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // World
    let mut world = World::new();
    world.light = Light {
        position: point![-10, 10, -10],
        intensity: color![1, 1, 1],
    };
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

    // Camera
    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform =
        Mat4::identity().view_transform(point![0, 1.5, -5], point![0, 1, 0], vector![0, 1, 0]);

    // Canvas
    let canvas = camera.render(&world);

    canvas.write("world_camera.ppm")?;
    Ok(())
}
