extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FPoint;
use sdl3::render::FRect;
use std::error::Error;
use std::time::Duration;

const PIXEL_WIDTH: u32 = 800;
const PIXEL_HEIGHT: u32 = 800;
const BLOCK_SIZE: usize = 1;
const WIDTH: usize = PIXEL_WIDTH as usize / BLOCK_SIZE;
const HEIGHT: usize = PIXEL_HEIGHT as usize / BLOCK_SIZE;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];
    seed(0, 600, 600, &mut grid);

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
                } => break 'running Ok(()),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    grid = tick_game(&grid);
                }
                _ => {}
            }
        }

        grid = tick_game(&grid);

        if BLOCK_SIZE > 1 {
            let rects = grid
                .iter()
                .enumerate()
                .flat_map(|(row, i)| {
                    i.iter().enumerate().filter_map(move |(column, j)| {
                        if *j {
                            Some(FRect::new(
                                (column * BLOCK_SIZE) as f32,
                                (row * BLOCK_SIZE) as f32,
                                BLOCK_SIZE as f32,
                                BLOCK_SIZE as f32,
                            ))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<FRect>>();

            if !rects.is_empty() {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.draw_rects(&rects)?;
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
        } else {
            let points = grid
                .iter()
                .enumerate()
                .flat_map(|(row, i)| {
                    i.iter().enumerate().filter_map(move |(column, j)| {
                        if *j {
                            Some(FPoint::new(column as f32, row as f32))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<FPoint>>();

            if !points.is_empty() {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.draw_points(points.as_slice())?;
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn seed(kind: u8, x: usize, y: usize, grid: &mut Vec<Vec<bool>>) {
    match kind {
        0 => {
            grid[y][x] = true;
            grid[1 + y][x] = true;
            grid[4 + y][x] = true;
            grid[y][1 + x] = true;
            grid[3 + y][1 + x] = true;
            grid[y][2 + x] = true;
            grid[3 + y][2 + x] = true;
            grid[4 + y][2 + x] = true;
            grid[2 + y][3 + x] = true;
            grid[y][4 + x] = true;
            grid[2 + y][4 + x] = true;
            grid[3 + y][4 + x] = true;
            grid[4 + y][4 + x] = true;
        }
        1 => {
            grid[y][x] = true;
            grid[1 + y][x] = true;
            grid[2 + y][x] = true;
            grid[3 + y][x] = true;
            grid[4 + y][x] = true;
            grid[5 + y][x] = true;
            grid[6 + y][x] = true;
            grid[7 + y][x] = true;
            grid[y][2 + x] = true;
            grid[1 + y][2 + x] = true;
            grid[2 + y][2 + x] = true;
            grid[3 + y][2 + x] = true;
            grid[4 + y][2 + x] = true;
            grid[5 + y][2 + x] = true;
            grid[6 + y][2 + x] = true;
            grid[7 + y][2 + x] = true;
            grid[y][1 + x] = true;
            grid[2 + y][1 + x] = true;
            grid[3 + y][1 + x] = true;
            grid[4 + y][1 + x] = true;
            grid[5 + y][1 + x] = true;
            grid[7 + y][1 + x] = true;
        }
        2 => {
            grid[y][x] = true;
            grid[1 + y][x] = true;
            grid[2 + y][x] = true;
        }
        _ => {}
    }
}

fn tick_game(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    (0..HEIGHT)
        .map(|i| {
            (0..WIDTH)
                .map(|j| {
                    let neighbours = directions.iter().fold(0, |acc, k| {
                        let row = (i as i32 + k.0) as usize;
                        let column = (j as i32 + k.1) as usize;
                        if 0 < row
                            && row < HEIGHT
                            && 0 < column
                            && column < WIDTH
                            && grid[row][column]
                        {
                            acc + 1
                        } else {
                            acc
                        }
                    });
                    if grid[i][j] {
                        (2..=3).contains(&neighbours)
                    } else {
                        neighbours == 3
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>()
}
