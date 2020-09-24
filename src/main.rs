use quicksilver::{
    geom::Vector,
    graphics::{Color, Image},
    run, Graphics, Input, Result, Settings, Window,
};

use boids::boids::Flock;

const WINDOW_SIZE: Vector = Vector { x: 800.0, y: 500.0 };
const MAX_BOIDS: u32 = 5;

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let img = Image::load(&gfx, "boid.png").await?;
    let img_size = img.size();

    let flock = Flock::new(MAX_BOIDS, WINDOW_SIZE, img_size);
    let mut boids = flock.boids;
    let area = flock.area;

    loop {
        while let Some(_) = input.next_event().await {}
        gfx.clear(Color::WHITE);

        let mut new_boids = Vec::new();
        for boid in boids.iter() {
            let mut b = *boid;

            b.detect_edges(area);
            b.movement();
            b.draw(&img, &mut gfx);

            new_boids.push(b);
        }

        boids = new_boids;
        gfx.present(&window)?;
    }
}

#[allow(unused_variables)]
fn main() {
    let settings = Settings {
        title: "Boids simulation",
        resizable: true,
        size: WINDOW_SIZE,
        ..Settings::default()
    };
    run(settings, app);
}
