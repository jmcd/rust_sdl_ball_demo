extern crate rand;
extern crate sdl2;

use self::rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::f64;

const X: usize = 0;
const Y: usize = 1;

struct Vertex {
    values: [f64; 2],
}

struct Ball {
    location: Vertex,
    velocity: Vertex,
    radius: f64,
    color: pixels::Color,
}

impl Ball {
    fn draw(&self, canvas: &Canvas<Window>) {
        canvas
            .filled_circle(
                self.location.values[X] as i16,
                self.location.values[Y] as i16,
                self.radius as i16,
                self.color,
            )
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    fn advance_in_bounds(&mut self, elapsed_seconds: f64, bounds: &Vertex) {
        self.advance(elapsed_seconds);

        for dim in X..=Y {
            if (self.location.values[dim] > bounds.values[dim] - self.radius)
                || ((self.location.values[dim] as u32) < (self.radius as u32))
            {
                self.velocity.values[dim] = self.velocity.values[dim] * -1.0;
                self.advance_dim(dim, elapsed_seconds);
            }
        }
    }

    fn advance(&mut self, elapsed_seconds: f64) {
        for dim in X..=Y {
            self.advance_dim(dim, elapsed_seconds)
        }
    }

    fn advance_dim(&mut self, dim: usize, elapsed_seconds: f64) {
        self.location.values[dim] =
            self.location.values[dim] + self.velocity.values[dim] * elapsed_seconds;
    }
}

fn random_ball(min_radius: u32, max_radius: u32, screen_size: &Vertex) -> Ball {
    let mut rng = rand::thread_rng();
    let v = rng.gen_range(100, 1000) as f64;

    let rn: f64 = rng.gen();
    let a = rn * f64::consts::PI * 2.0;

    let ball_color = pixels::Color::RGB(
        rng.gen_range(0x80, 0xff) as u8,
        rng.gen_range(0x80, 0xff) as u8,
        rng.gen_range(0x80, 0xff) as u8,
    );

    let rad = rng.gen_range(min_radius, max_radius);
    return Ball {
        location: Vertex {
            values: [
                rng.gen_range(rad, screen_size.values[X] as u32 - rad) as f64,
                rng.gen_range(rad, screen_size.values[Y] as u32 - rad) as f64,
            ],
        },
        velocity: Vertex {
            values: [a.cos() * v, a.sin() * v],
        },
        radius: rad as f64,
        color: ball_color,
    };
}

fn bounds(window: &Window) -> Vertex {
    let size = window.drawable_size();
    return Vertex {
        values: [size.0 as f64, size.1 as f64],
    };
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    let ball_inc = if args.len() > 1 {
        args[1].parse::<i32>().unwrap()
    } else {
        1
    };

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let window = video_subsys
        .window("ball", 800, 600)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let bg_color = pixels::Color::RGB(0, 0, 0);

    let mut balls: Vec<Ball> = Vec::new();

    let mut timer = sdl_context.timer()?;
    let mut events = sdl_context.event_pump()?;

    let mut previous = timer.ticks();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                }
                | Event::MouseButtonDown { .. } => {
                    let screen_size = bounds(&canvas.window_mut());
                    for _i in 0..ball_inc {
                        balls.push(random_ball(5, 30, &screen_size));
                    }
                    println!("There are {} balls!", balls.len())
                }
                _ => {}
            }
        }

        let current = timer.ticks();
        let elapsed_seconds = ((current - previous) as f64) / 1000.0;
        previous = current;

        canvas.set_draw_color(bg_color);
        canvas.clear();

        let screen_size = bounds(&canvas.window_mut());
        for ball in balls.iter_mut() {
            ball.draw(&canvas);
            ball.advance_in_bounds(elapsed_seconds, &screen_size);
        }

        canvas.present();
    }

    Ok(())
}
