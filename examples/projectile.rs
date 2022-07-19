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

fn main() {
    let mut p = Projectile {
        position: point![0, 1, 0],
        velocity: vector![1, 1, 0].normalize(),
    };
    let e = Environment {
        gravity: vector![0, -0.1, 0],
        wind: vector![-0.01, 0, 0],
    };
    let mut ticks_count: usize = 0;
    println!();
    println!("{:^5} {:^7} {:^7}", "Tick", "x", "y");
    println!("{:^5} {:^7} {:^7}", "-----", "-------", "-------");
    while p.position.y > 0.0 {
        println!(
            "{:^5} {:>7.2} {:>7.2}",
            ticks_count, p.position.x, p.position.y
        );
        p = tick(&e, p);
        ticks_count += 1;
    }
    println!(
        "It takes {} ticks for the projectile to hit the ground.\n",
        ticks_count - 1
    );
}
