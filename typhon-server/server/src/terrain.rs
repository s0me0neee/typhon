use anyhow::{Result, bail};
use noise::{NoiseFn, Perlin};
use std::fmt::Display;
use strum::{EnumIter, IntoEnumIterator};
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, EnumIter, TS, Serialize)]
#[ts(export, export_to = ts_file!())]
pub(crate) enum Biome {
    Plain,
    Water,
    Wood,
    Hill,
    Desert,
}

#[derive(Debug, Clone, Copy, TS, Serialize)]
#[ts(export, export_to = ts_file!())]
pub(crate) struct Terrain {
    biome: Biome,
    elevation: i32,
}

#[derive(Debug, Clone, Copy, TS, Serialize)]
#[ts(export, export_to = ts_file!())]
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

#[derive(Debug, TS, Serialize)]
#[ts(export, export_to = ts_file!())]
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
            bail!("World width have to be at least 2 and even, got: {}", w);
        }

        if h < 3 {
            bail!("World height have to be at least 3, got: {}", h);
        }

        let mut cells = Vec::with_capacity((w * h) as usize);

        for v in 0..h {
            for u in 0..w {
                cells.push(LandCell::new(u, v));
            }
        }

        let world = World {
            cells,
            width: w,
            height: h,
        };

        Ok(world)
    }

    fn index(&self, (u, v): (u32, u32)) -> usize {
        (v as usize) * (self.width as usize) + (u as usize)
    }
}

fn world_gen(world: &mut World, seed: u32) -> Result<()> {
    let perlin = Perlin::new(seed);
    let noice: Vec<Vec<f64>> = Vec::with_capacity(world.height as usize);
    let scale = 0.05;
    for y in 0..world.height {
        for x in 0..world.width {
            let mut v = Vec::with_capacity(world.width as usize);
            v.push(perlin.get([x as f64 * scale, y as f64 * scale]));
        }
    }
    Ok(())
}
