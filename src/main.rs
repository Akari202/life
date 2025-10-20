extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::keyboard::Scancode;
use sdl3::pixels::Color;
use std::error::Error;
use std::time::Duration;

use crate::game::Game;
use crate::game::Seed;

mod game;

const PIXEL_WIDTH: u32 = 800;
const PIXEL_HEIGHT: u32 = 800;
const BLOCK_SIZE: isize = 8;
const FPS: u32 = 60;
const TPS: u32 = 10;

const WIDTH: usize = PIXEL_WIDTH as usize / BLOCK_SIZE as usize;
const HEIGHT: usize = PIXEL_HEIGHT as usize / BLOCK_SIZE as usize;
const FPT: u32 = FPS / TPS;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut game = Game::new();
    game.seed(Seed::Acorn, 46, 30);
    game.seed(Seed::Acorn, 46, 90);
    game.seed(Seed::BLFive, 10, 150);
    game.seed(Seed::BLLine, 100, 130);
    game.seed(Seed::GosperGun, 100, 10);

    let mut paused: bool = true;
    let mut i: u32 = 0;
    let mut move_speed: isize = 1;
    let mut offset: (isize, isize) = (0, 0);

    let window = video_subsystem
        .window("Conway's Game of Life", PIXEL_WIDTH, PIXEL_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running Ok(()),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    game.tick();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    paused = !paused;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    move_speed = 5;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    move_speed = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    println!("There are {} live cells", game.count());
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    offset = (0, 0);
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        for i in keyboard_state.pressed_scancodes() {
            match i {
                Scancode::Up => {
                    offset.1 -= move_speed;
                }
                Scancode::Down => {
                    offset.1 += move_speed;
                }
                Scancode::Right => {
                    offset.0 += move_speed;
                }
                Scancode::Left => {
                    offset.0 -= move_speed;
                }
                _ => {}
            }
        }

        if !paused && i.is_multiple_of(FPT) {
            game.tick();
            i = 0;
        }
        i += 1;

        let rects = game.get_rects(offset);

        if !rects.is_empty() {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rects(&rects)?;
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
