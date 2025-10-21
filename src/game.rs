use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

use itertools::Itertools;
use sdl3::render::FRect;

use crate::BLOCK_SIZE;

pub struct Game<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + PartialEq
        + Eq
        + Debug
        + Clone
        + Copy
        + Hash
        + Mul<Output = T>
        + From<i8>
        + PartialOrd,
    i16: TryFrom<T>
{
    population: Vec<Cell<T>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + PartialEq
        + Eq
        + Debug
        + Clone
        + Copy
        + Hash
        + Mul<Output = T>
        + From<i8>
        + PartialOrd,
    i16: TryFrom<T>
{
    x: T,
    y: T
}

pub enum Seed {
    PentaDecathlon,
    Blinker,
    BLFive,
    BLLine,
    Acorn,
    GosperGun
}

impl<T> Game<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + PartialEq
        + Eq
        + Debug
        + Clone
        + Copy
        + Hash
        + Mul<Output = T>
        + From<i8>
        + PartialOrd,
    i16: TryFrom<T>
{
    pub fn new() -> Self {
        Self {
            population: Vec::new()
        }
    }

    fn spawn(&mut self, x: T, y: T) {
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
            .collect::<Vec<Cell<T>>>()
    }

    pub fn get_rects(&self, offset: (T, T), size: (T, T)) -> Vec<FRect> {
        self.population
            .iter()
            .filter_map(|i| i.rect(offset, size))
            .collect()
    }

    pub fn seed(&mut self, kind: Seed, x: T, y: T) {
        match kind {
            Seed::BLFive => {
                self.spawn(x, y);
                self.spawn(x, T::from(1i8) + y);
                self.spawn(x, T::from(4i8) + y);
                self.spawn(T::from(1i8) + x, y);
                self.spawn(T::from(1i8) + x, T::from(3i8) + y);
                self.spawn(T::from(2i8) + x, y);
                self.spawn(T::from(2i8) + x, T::from(3i8) + y);
                self.spawn(T::from(2i8) + x, T::from(4i8) + y);
                self.spawn(T::from(3i8) + x, T::from(2i8) + y);
                self.spawn(T::from(4i8) + x, y);
                self.spawn(T::from(4i8) + x, T::from(2i8) + y);
                self.spawn(T::from(4i8) + x, T::from(3i8) + y);
                self.spawn(T::from(4i8) + x, T::from(4i8) + y);
            }
            Seed::PentaDecathlon => {
                self.spawn(x, y);
                self.spawn(x, T::from(1i8) + y);
                self.spawn(x, T::from(2i8) + y);
                self.spawn(x, T::from(3i8) + y);
                self.spawn(x, T::from(4i8) + y);
                self.spawn(x, T::from(5i8) + y);
                self.spawn(x, T::from(6i8) + y);
                self.spawn(x, T::from(7i8) + y);
                self.spawn(T::from(2i8) + x, y);
                self.spawn(T::from(2i8) + x, T::from(1i8) + y);
                self.spawn(T::from(2i8) + x, T::from(2i8) + y);
                self.spawn(T::from(2i8) + x, T::from(3i8) + y);
                self.spawn(T::from(2i8) + x, T::from(4i8) + y);
                self.spawn(T::from(2i8) + x, T::from(5i8) + y);
                self.spawn(T::from(2i8) + x, T::from(6i8) + y);
                self.spawn(T::from(2i8) + x, T::from(7i8) + y);
                self.spawn(T::from(1i8) + x, y);
                self.spawn(T::from(1i8) + x, T::from(2i8) + y);
                self.spawn(T::from(1i8) + x, T::from(3i8) + y);
                self.spawn(T::from(1i8) + x, T::from(4i8) + y);
                self.spawn(T::from(1i8) + x, T::from(5i8) + y);
                self.spawn(T::from(1i8) + x, T::from(7i8) + y);
            }
            Seed::Blinker => {
                self.spawn(x, y);
                self.spawn(x, T::from(1i8) + y);
                self.spawn(x, T::from(2i8) + y);
            }
            Seed::Acorn => {
                self.spawn(x, T::from(2i8) + y);
                self.spawn(T::from(1i8) + x, y);
                self.spawn(T::from(1i8) + x, T::from(2i8) + y);
                self.spawn(T::from(3i8) + x, T::from(1i8) + y);
                self.spawn(T::from(4i8) + x, T::from(2i8) + y);
                self.spawn(T::from(5i8) + x, T::from(2i8) + y);
                self.spawn(T::from(6i8) + x, T::from(2i8) + y);
            }
            Seed::BLLine => {
                for i in 0..8i8 {
                    self.spawn(T::from(i) + x, y);
                }
                for i in 0..5i8 {
                    self.spawn(T::from(i) + T::from(9i8) + x, y);
                }
                for i in 0..3i8 {
                    self.spawn(T::from(i) + T::from(17i8) + x, y);
                }
                for i in 0..7i8 {
                    self.spawn(T::from(i) + T::from(26i8) + x, y);
                }
                for i in 0..5i8 {
                    self.spawn(T::from(i) + T::from(34i8) + x, y);
                }
            }
            Seed::GosperGun => {
                self.spawn(x, T::from(4i8) + y);
                self.spawn(x, T::from(5i8) + y);
                self.spawn(T::from(1i8) + x, T::from(4i8) + y);
                self.spawn(T::from(1i8) + x, T::from(5i8) + y);
                self.spawn(T::from(10i8) + x, T::from(4i8) + y);
                self.spawn(T::from(10i8) + x, T::from(5i8) + y);
                self.spawn(T::from(10i8) + x, T::from(6i8) + y);
                self.spawn(T::from(11i8) + x, T::from(3i8) + y);
                self.spawn(T::from(11i8) + x, T::from(7i8) + y);
                self.spawn(T::from(12i8) + x, T::from(2i8) + y);
                self.spawn(T::from(12i8) + x, T::from(8i8) + y);
                self.spawn(T::from(13i8) + x, T::from(2i8) + y);
                self.spawn(T::from(13i8) + x, T::from(8i8) + y);
                self.spawn(T::from(14i8) + x, T::from(5i8) + y);
                self.spawn(T::from(15i8) + x, T::from(3i8) + y);
                self.spawn(T::from(15i8) + x, T::from(7i8) + y);
                self.spawn(T::from(16i8) + x, T::from(4i8) + y);
                self.spawn(T::from(16i8) + x, T::from(5i8) + y);
                self.spawn(T::from(16i8) + x, T::from(6i8) + y);
                self.spawn(T::from(17i8) + x, T::from(5i8) + y);
                self.spawn(T::from(20i8) + x, T::from(2i8) + y);
                self.spawn(T::from(20i8) + x, T::from(3i8) + y);
                self.spawn(T::from(20i8) + x, T::from(4i8) + y);
                self.spawn(T::from(21i8) + x, T::from(2i8) + y);
                self.spawn(T::from(21i8) + x, T::from(3i8) + y);
                self.spawn(T::from(21i8) + x, T::from(4i8) + y);
                self.spawn(T::from(22i8) + x, T::from(1i8) + y);
                self.spawn(T::from(22i8) + x, T::from(5i8) + y);
                self.spawn(T::from(24i8) + x, y);
                self.spawn(T::from(24i8) + x, T::from(1i8) + y);
                self.spawn(T::from(24i8) + x, T::from(5i8) + y);
                self.spawn(T::from(24i8) + x, T::from(6i8) + y);
                self.spawn(T::from(34i8) + x, T::from(2i8) + y);
                self.spawn(T::from(34i8) + x, T::from(3i8) + y);
                self.spawn(T::from(35i8) + x, T::from(2i8) + y);
                self.spawn(T::from(35i8) + x, T::from(3i8) + y);
            }
        }
    }
}

impl<T> Cell<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + PartialEq
        + Eq
        + Debug
        + Clone
        + Copy
        + Hash
        + Mul<Output = T>
        + From<i8>
        + PartialOrd,
    i16: TryFrom<T>
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // WARN: This function is totally unsafe and *will* cause undefined behavior
    pub fn rect(&self, offset: (T, T), size: (T, T)) -> Option<FRect> {
        if T::from(0i8) <= self.x - offset.0
            && self.x - offset.0 <= size.0
            && T::from(0i8) <= self.y - offset.1
            && self.y - offset.1 <= size.1
        {
            Some(FRect::new(
                f32::from(unsafe {
                    i16::try_from((self.x - offset.0) * T::from(BLOCK_SIZE)).unwrap_unchecked()
                }),
                f32::from(unsafe {
                    i16::try_from((self.y - offset.1) * T::from(BLOCK_SIZE)).unwrap_unchecked()
                }),
                BLOCK_SIZE as f32,
                BLOCK_SIZE as f32
            ))
        } else {
            None
        }
    }

    pub fn expand(&self) -> Vec<Cell<T>> {
        let directions: [(i8, i8); 9] = [
            (0, 0),
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1)
        ];
        directions
            .into_iter()
            .map(|(i, j)| Cell::new(T::from(i) + self.x, T::from(j) + self.y))
            .collect()
    }
}
