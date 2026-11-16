use std::fmt::Display;

use anyhow::{Result, bail};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumIter)]
pub(crate) enum Biome {
    Plain,
    Water,
    Wood,
    Hill,
    Desert,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Terrain {
    biome: Biome,
    elevation: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LandCell {
    pub(crate) terrain: Terrain,
    pub(crate) u: u32,
    pub(crate) v: u32,
}

impl LandCell {
    fn new(u: u32, v: u32) -> LandCell {
        let biome = Biome::iter()
            .nth(rand::random_range(0..Biome::iter().len()))
            .unwrap();
        let terrain = Terrain {
            biome,
            elevation: rand::random_range(-30..210),
        };
        LandCell { terrain, u, v }
    }
}

/*
 * w ---------------
 *|   * * * * * * * *
 *|    * * * * * * *
 *|   * * * * * * * *
 *|    * * * * * * *
 *|   * * * * * * * *
 *h
 *
 * each cell can be pointed by two vector
 * u is ->
 * v is \ or /
 * cord is (u,v)
 */

#[derive(Debug)]
pub(crate) struct World {
    cells: Vec<LandCell>,
    width: u32,
    height: u32,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell_char = |u: u32, v: u32| -> char {
            match self.cells[World::index(self, (u, v))].terrain.biome {
                Biome::Plain => '.',
                Biome::Water => '~',
                Biome::Wood => '#',
                Biome::Hill => '^',
                Biome::Desert => '"',
            }
        };
        for v in 0..self.height {
            if v % 2 == 0 {
                for u in 0..self.width {
                    write!(f, "{} ", cell_char(u, v))?;
                }
            } else {
                write!(f, " ")?;
                for u in 0..self.width {
                    write!(f, "{} ", cell_char(u, v))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl World {
    pub fn new(w: u32, h: u32) -> Result<World> {
        if w < 2 || w % 2 != 0 {
            bail!("World width have to be at least 2, got: {}", w);
        }

        if h < 3 {
            bail!("World height have to be at least 3, got: {}", h);
        }

        let mut cells = Vec::with_capacity((w * h) as usize);
        for u in 0..w {
            for v in 0..h {
                cells.push(LandCell::new(u, v));
            }
        }
        Ok(World {
            cells,
            width: w,
            height: h,
        })
    }
    fn index(&self, (u, v): (u32, u32)) -> usize {
        (v as usize) * (self.width as usize) + (u as usize)
    }
}
