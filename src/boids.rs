use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Image,
    Graphics,
};

use rand::Rng;

const SPEED: f32 = 1.0;

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
        let rand: f32 = rand::thread_rng().gen();
        let velocity = Vector::from_angle(rand * 360.0).normalize() * SPEED;

        Boid {
            id: rand::thread_rng().gen(),
            position: pos,
            acceleration: Vector::ZERO,
            velocity: velocity,
            max_force: 1.0,
            max_speed: 4.0,
        }
    }

    pub fn edges(&mut self, area: Vector) {
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

    pub fn fly(&mut self) {
        self.position += self.velocity * SPEED;
        self.velocity += self.acceleration;
        self.acceleration = Vector::ZERO;
    }

    pub fn update(&mut self, area: Vector, _boids: Vec<Boid>) {
        self.fly();
        self.edges(area);
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
            boids: boids,
            img_size: img_size,
            max_boids: max_boids,
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
