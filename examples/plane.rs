use std::f64::consts::PI;
use std::rc::Rc;
use trace::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let side = 5;

    let mut floor = Plane::default();
    floor.transform = Mat4::identity();
    floor.material = Material::new();
    floor.material.color = color![1, 0.9, 0.9];
    floor.material.specular = 0.0;

    let mut one = Plane::default();
    one.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 6.0)
        .translate(0, 0, side);
    one.material = Material::new();
    one.material.color = color![1, 0.9, 0.9];
    one.material.specular = 0.0;

    let mut one_mirror = Plane::default();
    one_mirror.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / -6.0)
        .translate(0, 0, side);
    one_mirror.material = Material::new();
    one_mirror.material.color = color![1, 0.9, 0.9];
    one_mirror.material.specular = 0.0;

    let mut two = Plane::default();
    two.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 2.0)
        .translate(side, 0, 0);
    two.material = Material::new();
    two.material.color = color![1, 0.9, 0.9];
    two.material.specular = 0.0;

    let mut two_mirror = Plane::default();
    two_mirror.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 2.0)
        .translate(-side, 0, 0);
    two_mirror.material = Material::new();
    two_mirror.material.color = color![1, 0.9, 0.9];
    two_mirror.material.specular = 0.0;

    let mut three = Plane::default();
    three.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / -6.0)
        .translate(0, 0, -side);
    three.material = Material::new();
    three.material.color = color![1, 0.9, 0.9];
    three.material.specular = 0.0;

    let mut three_mirror = Plane::default();
    three_mirror.transform = Mat4::identity()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 6.0)
        .translate(0, 0, -side);
    three_mirror.material = Material::new();
    three_mirror.material.color = color![1, 0.9, 0.9];
    three_mirror.material.specular = 0.0;

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
        position: point![-3, 5, -1],
        intensity: color![1, 1, 1],
    };
    world.objects = vec![
        Rc::new(floor),
        Rc::new(one),
        Rc::new(one_mirror),
        Rc::new(two),
        Rc::new(two_mirror),
        Rc::new(three),
        Rc::new(three_mirror),
        Rc::new(middle),
        Rc::new(right),
        Rc::new(left),
    ];

    // Camera
    let mut camera = Camera::new(100, 100, PI / 3.0);
    camera.transform =
        Mat4::identity().view_transform(point![0, 4, 0], point![0.01, 0, 0], vector![0, 3, 1]);

    // Canvas
    let canvas = camera.render(&world);

    canvas.write("plane.ppm")?;
    Ok(())
}
