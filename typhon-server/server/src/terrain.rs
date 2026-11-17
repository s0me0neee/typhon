use crate::ts_file;
use anyhow::{Result, bail};
use noise::{NoiseFn, Perlin};
use serde::Serialize;
use std::{fmt::Display, ops::Range};
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

        let mut world = World {
            cells,
            width: w,
            height: h,
        };

        let conf = WorldGenConf {
            elevation_range: -30.0..210.0,
            perlin_noise_seed: 1,
            perlin_noise_layers: &[0.03, 0.08, 0.1],
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
pub(crate) struct WorldGenConf<'a> {
    pub(crate) elevation_range: Range<f64>,
    pub(crate) perlin_noise_seed: u32,
    pub(crate) perlin_noise_layers: &'a [f32],
}

fn world_gen(world: &mut World, conf: WorldGenConf) -> Result<()> {
    let perlin = Perlin::new(conf.perlin_noise_seed);

    let rescale =
        |r: &Range<f64>, n: f64| -> f64 { r.start + n * (r.end - r.start) };

    for scale in conf.perlin_noise_layers {
        let mut noise: Vec<Vec<f64>> =
            Vec::with_capacity(world.height as usize);
        for y in 0..world.height {
            let mut v = Vec::with_capacity(world.width as usize);
            for x in 0..world.width {
                let val = perlin
                    .get([x as f64 * *scale as f64, y as f64 * *scale as f64]);
                v.push((val + 1.0) / 2.0);
            }
            noise.push(v);
        }
        for (v, row) in noise.iter().enumerate() {
            for (u, &val) in row.iter().enumerate() {
                let i = world.index((u as u32, v as u32));
                world.cells[i].terrain.elevation =
                    rescale(&conf.elevation_range, val) as f32;
            }
        }
    }

    Ok(())
}

#[test]
fn world_gen_test() -> Result<()> {
    let mut world = World::new(30, 15)?;
    let conf = WorldGenConf {
        elevation_range: -30.0..210.0,
        perlin_noise_seed: 1,
        perlin_noise_layers: &[0.1, 0.2, 0.3],
    };
    world_gen(&mut world, conf)?;
    Ok(())
}
