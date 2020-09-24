use quicksilver::{geom::Rectangle, graphics::Image, Graphics};

use quicksilver::geom::Vector;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Boid {
    pub id: u64,
    pub max_force: f32,
    pub max_speed: f32,
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
}

impl Boid {
    pub fn new(pos: Vector) -> Boid {
        Boid {
            id: rand::thread_rng().gen(),
            position: pos,
            acceleration: Vector::ZERO,
            velocity: Vector::new(2.0, 2.0),
            max_force: 1.0,
            max_speed: 10.0,
        }
    }

    pub fn detect_edges(&mut self, area: Vector) {
        let mut pos = self.position;

        if pos.x < 0.0 {
            pos.x = area.x;
        }
        if pos.x > area.x {
            pos.x = 0.0;
        }

        if pos.y < 0.0 {
            pos.y = area.y;
        }
        if pos.y > area.y {
            pos.y = 0.0;
        }

        self.position = pos;
    }

    pub fn movement(&mut self) {
        self.position += self.velocity;
        self.position += self.acceleration;
    }

    pub fn draw(&self, img: &Image, gfx: &mut Graphics) {
        let rect = Rectangle::new(self.position, img.size());
        gfx.draw_image(&img, rect);
    }
}

pub struct Flock {
    pub area: Vector,
    pub max_boids: u32,
    pub boids: Vec<Boid>,
    pub img_size: Vector,
}

impl Flock {
    pub fn new(max_boids: u32, area: Vector, img_size: Vector) -> Flock {
        let boids = Vec::with_capacity(max_boids as usize);

        let mut flock = Flock {
            area: area,
            max_boids: max_boids,
            boids: boids,
            img_size: img_size,
        };

        for _ in 0..max_boids {
            flock.new_boid(img_size);
        }

        return flock;
    }

    pub fn new_boid(&mut self, img_size: Vector) {
        let rand_x = rand::thread_rng().gen_range(0.0, self.area.x - img_size.x);
        let rand_y = rand::thread_rng().gen_range(0.0, self.area.y - img_size.y);
        let pos = Vector::new(rand_x, rand_y);
        let boid = Boid::new(pos);
        self.boids.push(boid);
    }
}
