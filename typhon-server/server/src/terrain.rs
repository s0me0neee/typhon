use crate::ts_file;
use anyhow::{Result, bail};
use serde::Serialize;
use std::{fmt::Display, ops::Range};
use ts_rs::TS;

// credits to: https://www.redblobgames.com/maps/terrain-from-noise/

#[derive(Debug, Clone, Copy, strum::EnumIter, TS, Serialize)]
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
    pub(crate) biome: Biome,
    pub(crate) elevation: f32,
}

#[derive(Debug, Clone, Copy, TS, Serialize)]
#[ts(export, export_to = ts_file!())]
pub(crate) struct LandCell {
    pub(crate) terrain: Terrain,
    pub(crate) temp: f32,
    pub(crate) u: u32,
    pub(crate) v: u32,
}

impl LandCell {
    fn new(u: u32, v: u32) -> LandCell {
        let biome = Biome::Plain;
        let terrain = Terrain {
            biome,
            elevation: 0.0,
        };
        LandCell {
            terrain,
            temp: 0.0,
            u,
            v,
        }
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
    pub(crate) cells: Vec<LandCell>,
    pub(crate) width: u32,
    pub(crate) height: u32,
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
    pub fn new(w: u32, h: u32, conf: &WorldGenConf) -> Result<World> {
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

        let mut world = World {
            cells,
            width: w,
            height: h,
        };

        crate::world_gen::world_gen(&mut world, conf)?;

        Ok(world)
    }

    fn index(&self, (u, v): (u32, u32)) -> usize {
        (v as usize) * (self.width as usize) + (u as usize)
    }
}

// ops::Range because serde and ts-rs only support this
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = ts_file!())]
pub(crate) struct WorldGenConf {
    /// range of min and max elevation
    pub(crate) elevation_range: Range<f64>,
    pub(crate) perlin_noise_seed: u32,
    /// number of noise layers
    pub(crate) perlin_octaves: usize,
    /// the amount of increase in amp per octave
    pub(crate) perlin_frequency: f64,
    /// lower makes smoother
    pub(crate) perlin_persistence: f64,
    /// temp at equator
    pub(crate) max_temp: f32,
    /// temp at polls
    pub(crate) min_temp: f32,
    /// how much colder the highest peak is than sea level
    pub(crate) elevation_temp_drop: f32,
}
