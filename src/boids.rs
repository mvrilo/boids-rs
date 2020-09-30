use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Image,
    Graphics,
};
use rand::Rng;

const MAX_FORCE: f32 = 1.1;
const MAX_SPEED: f32 = 4.0;

const ALIGN_AREA: f32 = 95.0;
const COHESION_AREA: f32 = 90.0;
const SEPARATION_AREA: f32 = 50.0;

const NEARBY_RADIUS: f32 = 40.0;

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
    pub fn new(pos: Vector) -> Self {
        let rand: f32 = rand::thread_rng().gen();
        let velocity = Vector::from_angle(rand * 360.0);

        Self {
            id: rand::thread_rng().gen(),
            position: pos,
            acceleration: Vector::ZERO,
            velocity: velocity,
            max_force: MAX_FORCE,
            max_speed: MAX_SPEED,
        }
    }

    pub fn distance(&mut self, boid: &Boid) -> f32 {
        self.position.distance(boid.position)
    }

    pub fn nearby(&mut self, radius: f32, boids: Vec<Boid>) -> Vec<Boid> {
        let id = self.id;
        boids
            .iter()
            .filter(|boid| id != boid.id)
            .filter(|boid| self.distance(boid) < radius)
            .map(|boid| *boid)
            .collect()
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
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.acceleration = Vector::ZERO;
    }

    pub fn behave(&mut self, boids: Vec<Boid>) {
        let (mut align_count, mut align) = (0, Vector::ZERO);
        let (mut cohesion_count, mut cohesion) = (0, Vector::ZERO);
        let (mut separation_count, mut separation) = (0, Vector::ZERO);

        self.nearby(NEARBY_RADIUS, boids).iter().for_each(|boid| {
            let dist = self.distance(boid);
            if dist < ALIGN_AREA {
                align += boid.velocity;
                align_count += 1;
            }
            if dist < COHESION_AREA {
                cohesion += boid.position;
                cohesion_count += 1;
            }
            if dist < SEPARATION_AREA {
                let mut pos = self.position;
                pos -= boid.position;
                pos /= dist;
                separation += pos;
                separation_count += 1;
            }
        });

        let mut rules: Vec<Vector> = Vec::new();

        if align_count > 0 {
            align /= align_count as f32;
            align = align.normalize();
            align *= self.max_speed * 0.8;
            align -= self.velocity;

            let len = align.len();
            if len > self.max_force {
                align /= len;
                align *= self.max_force;
            }
            rules.push(align);
        }

        if cohesion_count > 0 {
            cohesion /= cohesion_count as f32;
            cohesion -= self.position;
            cohesion = cohesion.normalize();
            cohesion *= self.max_speed;
            cohesion -= self.velocity;
            cohesion = cohesion.normalize();
            cohesion *= self.max_force;
            rules.push(cohesion);
        }

        if separation_count > 0 {
            separation /= separation_count as f32;
            separation = separation.normalize();
            separation *= self.max_speed;
            separation -= self.velocity;
            separation = separation.normalize();
            separation *= self.max_force;
            rules.push(separation);
        }

        let rules_count = rules.len();
        if rules_count > 0 {
            rules.iter().for_each(|acc| self.acceleration += *acc);
            self.acceleration /= rules_count as f32;
        }
    }

    pub fn update(&mut self, area: Vector, boids: Vec<Boid>) {
        self.edges(area);
        self.behave(boids);
        self.fly();
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
