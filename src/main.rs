mod tuple;
mod color;

struct Environment {
  gravity: tuple::Vector,
  wind: tuple::Vector
}

struct Projectile {
  position: tuple::Point,
  velocity: tuple::Vector
}

fn tick (environment: &Environment, projectile: Projectile) -> Projectile {
  Projectile {
    position: projectile.position + projectile.velocity,
    velocity: projectile.velocity + environment.gravity + environment.wind
  }
}

fn main () {
  let mut p = Projectile {
    position: tuple::helpers::point(0.0, 1.0, 0.0),
    velocity: tuple::ops::normalize(tuple::helpers::vector(1.0, 1.0, 0.0))
  };

  let e = Environment {
    gravity: tuple::helpers::vector(0.0, -0.1, 0.0),
    wind: tuple::helpers::vector(-0.01, 0.0, 0.0)
  };

  while p.position.y > 0.0 {
    p = tick(&e, p);
    println!("{:?}, {:?}, {:?}", p.position.x, p.position.y, p.position.z);
  }
}
