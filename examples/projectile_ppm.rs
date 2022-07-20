use std::fs::File;
use std::io::Write;
use trace::prelude::*;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut p = Projectile {
        position: point![0, 1, 0],
        velocity: vector![1, 1.8, 0].normalize() * 11.25,
    };
    let e = Environment {
        gravity: vector![0, -0.1, 0],
        wind: vector![-0.01, 0, 0],
    };
    let mut canvas = Canvas::new(900, 550);
    let mut ticks_count: usize = 0;
    println!();
    println!("{:^5} {:^7} {:^7}", "Tick", "x", "y");
    println!("{:^5} {:^7} {:^7}", "-----", "-------", "-------");
    while p.position.y > 0.0 {
        println!(
            "{:^5} {:>7.2} {:>7.2}",
            ticks_count, p.position.x, p.position.y
        );
        // EXPERIMENT

        if (p.position.y.ceil() as usize) < canvas.height {
            let x = p.position.x as usize;
            let y = canvas.height - p.position.y.ceil() as usize;
            if x < canvas.width && y < canvas.height {
                canvas[(x, y)] = color![1, 0, 0];
            }
        }
        p = tick(&e, p);
        ticks_count += 1;
    }
    println!(
        "It takes {} ticks for the projectile to hit the ground.\n",
        ticks_count - 1
    );
    let mut file = File::create("projectile.ppm")?;
    file.write(&canvas.to_ppm().into_bytes())?;
    Ok(())
}
