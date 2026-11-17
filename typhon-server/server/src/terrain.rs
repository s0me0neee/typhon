use crate::ts_file;
use anyhow::{Result, bail};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use serde::Serialize;
use std::{f64::consts::TAU, fmt::Display, ops::Range};
use strum::{EnumIter, IntoEnumIterator};
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
        let biome = Biome::iter()
            .nth(rand::random_range(0..Biome::iter().len()))
            .unwrap();
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
    pub(crate) elevation_range: Range<f64>,
    pub(crate) perlin_noise_seed: u32,
    pub(crate) octaves: usize,
    pub(crate) frequency: f64,
    pub(crate) persistence: f64,
}

impl Default for WorldGenConf {
    fn default() -> Self {
        WorldGenConf {
            elevation_range: -30.0..210.0,
            perlin_noise_seed: 1,
            octaves: 4,
            frequency: 0.05,
            persistence: 0.5,
        }
    }
}

fn world_gen(world: &mut World, conf: &WorldGenConf) -> Result<()> {
    let fbm = Fbm::<Perlin>::new(conf.perlin_noise_seed)
        .set_octaves(conf.octaves)
        .set_frequency(conf.frequency)
        .set_persistence(conf.persistence);
    let w = world.width as f64;

    let radius = w / TAU;
    let r = &conf.elevation_range;

    for v in 0..world.height {
        for u in 0..world.width {
            let px = u as f64 + 0.5 * (v % 2) as f64;
            let py = v as f64 * 3f64.sqrt() / 2.0;
            let theta = TAU * px / w;
            let n = fbm
                .get([radius * theta.cos(), radius * theta.sin(), py])
                .clamp(-1.0, 1.0);

            let t = ((n + 1.0) / 2.0).powf(0.8);
            let i = world.index((u, v));
            world.cells[i].terrain.elevation =
                (r.start + t * (r.end - r.start)) as f32;
        }
    }

    Ok(())
}

#[test]
fn world_gen_test() -> Result<()> {
    let conf = WorldGenConf::default();
    let world = World::new(30, 15, &conf)?;
    let r = &conf.elevation_range;
    for c in &world.cells {
        let e = c.terrain.elevation as f64;
        assert!(r.start <= e && e <= r.end, "elevation {e} outside {r:?}");
    }
    Ok(())
}
