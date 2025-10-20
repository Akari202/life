use itertools::Itertools;
use sdl3::render::FRect;

use crate::{BLOCK_SIZE, HEIGHT, WIDTH};

pub struct Game {
    population: Vec<Cell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: isize,
    y: isize,
}

pub enum Seed {
    PentaDecathlon,
    Blinker,
    BLFive,
    BLLine,
    Acorn,
    GosperGun,
}

impl Game {
    pub fn new() -> Self {
        Self {
            population: Vec::new(),
        }
    }

    fn spawn(&mut self, x: isize, y: isize) {
        self.population.push(Cell::new(x, y));
    }

    pub fn count(&self) -> usize {
        self.population.len()
    }

    pub fn tick(&mut self) {
        let counts = self.population.iter().flat_map(|i| i.expand()).counts();
        self.population = counts
            .iter()
            .filter_map(|(i, j)| {
                if *j == 3usize || (*j == 4usize && self.population.contains(i)) {
                    Some(*i)
                } else {
                    None
                }
            })
            .collect::<Vec<Cell>>()
    }

    pub fn get_rects(&self, offset: (isize, isize)) -> Vec<FRect> {
        self.population
            .iter()
            .filter_map(|i| i.rect(offset))
            .collect()
    }

    pub fn seed(&mut self, kind: Seed, x: isize, y: isize) {
        match kind {
            Seed::BLFive => {
                self.spawn(x, y);
                self.spawn(x, 1 + y);
                self.spawn(x, 4 + y);
                self.spawn(1 + x, y);
                self.spawn(1 + x, 3 + y);
                self.spawn(2 + x, y);
                self.spawn(2 + x, 3 + y);
                self.spawn(2 + x, 4 + y);
                self.spawn(3 + x, 2 + y);
                self.spawn(4 + x, y);
                self.spawn(4 + x, 2 + y);
                self.spawn(4 + x, 3 + y);
                self.spawn(4 + x, 4 + y);
            }
            Seed::PentaDecathlon => {
                self.spawn(x, y);
                self.spawn(x, 1 + y);
                self.spawn(x, 2 + y);
                self.spawn(x, 3 + y);
                self.spawn(x, 4 + y);
                self.spawn(x, 5 + y);
                self.spawn(x, 6 + y);
                self.spawn(x, 7 + y);
                self.spawn(2 + x, y);
                self.spawn(2 + x, 1 + y);
                self.spawn(2 + x, 2 + y);
                self.spawn(2 + x, 3 + y);
                self.spawn(2 + x, 4 + y);
                self.spawn(2 + x, 5 + y);
                self.spawn(2 + x, 6 + y);
                self.spawn(2 + x, 7 + y);
                self.spawn(1 + x, y);
                self.spawn(1 + x, 2 + y);
                self.spawn(1 + x, 3 + y);
                self.spawn(1 + x, 4 + y);
                self.spawn(1 + x, 5 + y);
                self.spawn(1 + x, 7 + y);
            }
            Seed::Blinker => {
                self.spawn(x, y);
                self.spawn(x, 1 + y);
                self.spawn(x, 2 + y);
            }
            Seed::Acorn => {
                self.spawn(x, 2 + y);
                self.spawn(1 + x, y);
                self.spawn(1 + x, 2 + y);
                self.spawn(3 + x, 1 + y);
                self.spawn(4 + x, 2 + y);
                self.spawn(5 + x, 2 + y);
                self.spawn(6 + x, 2 + y);
            }
            Seed::BLLine => {
                for i in 0..8 {
                    self.spawn(i + x, y);
                }
                for i in 0..5 {
                    self.spawn(i + 9 + x, y);
                }
                for i in 0..3 {
                    self.spawn(i + 17 + x, y);
                }
                for i in 0..7 {
                    self.spawn(i + 26 + x, y);
                }
                for i in 0..5 {
                    self.spawn(i + 34 + x, y);
                }
            }
            Seed::GosperGun => {
                self.spawn(x, 4 + y);
                self.spawn(x, 5 + y);
                self.spawn(1 + x, 4 + y);
                self.spawn(1 + x, 5 + y);
                self.spawn(10 + x, 4 + y);
                self.spawn(10 + x, 5 + y);
                self.spawn(10 + x, 6 + y);
                self.spawn(11 + x, 3 + y);
                self.spawn(11 + x, 7 + y);
                self.spawn(12 + x, 2 + y);
                self.spawn(12 + x, 8 + y);
                self.spawn(13 + x, 2 + y);
                self.spawn(13 + x, 8 + y);
                self.spawn(14 + x, 5 + y);
                self.spawn(15 + x, 3 + y);
                self.spawn(15 + x, 7 + y);
                self.spawn(16 + x, 4 + y);
                self.spawn(16 + x, 5 + y);
                self.spawn(16 + x, 6 + y);
                self.spawn(17 + x, 5 + y);
                self.spawn(20 + x, 2 + y);
                self.spawn(20 + x, 3 + y);
                self.spawn(20 + x, 4 + y);
                self.spawn(21 + x, 2 + y);
                self.spawn(21 + x, 3 + y);
                self.spawn(21 + x, 4 + y);
                self.spawn(22 + x, 1 + y);
                self.spawn(22 + x, 5 + y);
                self.spawn(24 + x, y);
                self.spawn(24 + x, 1 + y);
                self.spawn(24 + x, 5 + y);
                self.spawn(24 + x, 6 + y);
                self.spawn(34 + x, 2 + y);
                self.spawn(34 + x, 3 + y);
                self.spawn(35 + x, 2 + y);
                self.spawn(35 + x, 3 + y);
            }
        }
    }
}

impl Cell {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn rect(&self, offset: (isize, isize)) -> Option<FRect> {
        if 0 <= self.x - offset.0
            && self.x - offset.0 <= WIDTH as isize
            && 0 <= self.y - offset.1
            && self.y - offset.1 <= HEIGHT as isize
        {
            Some(FRect::new(
                ((self.x - offset.0) * BLOCK_SIZE) as f32,
                ((self.y - offset.1) * BLOCK_SIZE) as f32,
                BLOCK_SIZE as f32,
                BLOCK_SIZE as f32,
            ))
        } else {
            None
        }
    }

    pub fn expand(&self) -> Vec<Cell> {
        let directions: [(i32, i32); 9] = [
            (0, 0),
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        directions
            .iter()
            .map(|(i, j)| Cell::new((i + self.x as i32) as isize, (j + self.y as i32) as isize))
            .collect()
    }
}
