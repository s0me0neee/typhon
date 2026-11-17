use crate::ts_file;
use anyhow::{Result, bail};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use serde::Serialize;
use std::{f64::consts::TAU, fmt::Display, ops::Range};
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
    biome: Biome,
    elevation: f32,
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
        let biome = Biome::Plain;
        let terrain = Terrain {
            biome,
            elevation: 0.0,
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

        world_gen(&mut world, conf)?;

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
    pub(crate) octaves: usize,
    /// the amount of increase in amp per octave
    pub(crate) frequency: f64,
    /// lower makes smoother
    pub(crate) persistence: f64,
}

fn world_gen(world: &mut World, conf: &WorldGenConf) -> Result<()> {
    let fbm = Fbm::<Perlin>::new(conf.perlin_noise_seed)
        .set_octaves(conf.octaves)
        .set_frequency(conf.frequency)
        .set_persistence(conf.persistence);
    let w = world.width as f64;

    let radius = w / TAU;
    let elevation_range = &conf.elevation_range;

    let mut noise = Vec::with_capacity(world.cells.len());
    for v in 0..world.height {
        for u in 0..world.width {
            let px = u as f64 + 0.5 * (v % 2) as f64;
            let py = v as f64 * 3f64.sqrt() / 2.0;
            let theta = TAU * px / w;
            noise.push(fbm.get([
                radius * theta.cos(),
                radius * theta.sin(),
                py,
            ]));
        }
    }

    let (lo, hi) = noise
        .iter()
        .fold((f64::MAX, f64::MIN), |(lo, hi), &n| (lo.min(n), hi.max(n)));
    let span = (hi - lo).max(f64::EPSILON);

    for (cell, n) in world.cells.iter_mut().zip(noise) {
        let t = ((n - lo) / span).powf(1.0);
        cell.terrain.elevation = (elevation_range.start
            + t * (elevation_range.end - elevation_range.start))
            as f32;
    }

    let mut elevations: Vec<f32> =
        world.cells.iter().map(|c| c.terrain.elevation).collect();
    elevations.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let qot = |ratio: f64| {
        elevations[((elevations.len() - 1) as f64 * ratio) as usize]
    };

    let water = qot(0.42);
    let plain = qot(0.78);
    let wood = qot(0.92);

    for cell in &mut world.cells {
        let e = cell.terrain.elevation;
        cell.terrain.biome = if e < water {
            Biome::Water
        } else if e < plain {
            Biome::Plain
        } else if e < wood {
            Biome::Wood
        } else {
            Biome::Hill
        };
    }

    Ok(())
}

#[test]
fn world_gen_test() -> Result<()> {
    let conf = WorldGenConf {
        perlin_noise_seed: rand::random(),
        elevation_range: -30.0..210.0,
        octaves: 4,
        frequency: 0.05,
        persistence: 0.5,
    };
    let world = World::new(30, 15, &conf)?;
    let r = &conf.elevation_range;
    for c in &world.cells {
        let e = c.terrain.elevation as f64;
        assert!(r.start <= e && e <= r.end, "elevation {e} outside {r:?}");
    }
    Ok(())
}
