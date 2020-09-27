extern crate quicksilver;

use boids::boids::*;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Image},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};

const WINDOW_SIZE: Vector = Vector { x: 800.0, y: 500.0 };

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    const MAX_BOIDS: u32 = 100;

    let img = Image::load(&gfx, "boid.png").await?;
    let img_size = img.size();

    let flock = Flock::new(MAX_BOIDS, WINDOW_SIZE, img_size);
    let mut boids = flock.boids;
    let area = flock.area;

    loop {
        if let Some(ev) = input.next_event().await {
            if let Event::KeyboardInput(key_event) = &ev {
                if key_event.key() == Key::Escape && key_event.is_down() {
                    return Ok(());
                }
            }
        }

        gfx.clear(Color::WHITE);

        boids = boids
            .iter()
            .map(|boid| *boid)
            .map(|mut boid| {
                boid.update(area, boids.clone());
                boid.draw(&img, &mut gfx);
                return boid;
            })
            .collect();

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
