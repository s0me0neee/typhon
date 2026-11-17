use crate::terrain::Biome;
use crate::terrain::World;
use crate::terrain::WorldGenConf;
use anyhow::Result;
use noise::Fbm;
use noise::MultiFractal;
use noise::NoiseFn;
use noise::Perlin;
use std::f64::consts::TAU;

pub fn world_gen(world: &mut World, conf: &WorldGenConf) -> Result<()> {
    let fbm = Fbm::<Perlin>::new(conf.perlin_noise_seed)
        .set_octaves(conf.perlin_octaves)
        .set_frequency(conf.perlin_frequency)
        .set_persistence(conf.perlin_persistence);

    let radius = world.width as f64 / TAU;
    let elevation_range = &conf.elevation_range;

    let mut noise = Vec::with_capacity(world.cells.len());
    for v in 0..world.height {
        for u in 0..world.width {
            let px = u as f64 + 0.5 * (v % 2) as f64;
            let py = v as f64 * 3f64.sqrt() / 2.0;
            let theta = TAU * px / world.width as f64;
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
    {
        let peak = qot(0.98);
        for cell in &mut world.cells {
            let mid = (world.height - 1) as f32 / 2.0;
            let eq = 1.0 - (cell.v as f32 - mid).abs() / (mid);
            let above_sea = ((cell.terrain.elevation - water)
                / (peak - water).max(f32::EPSILON))
            .max(0.0);
            cell.temp = conf.min_temp + (conf.max_temp - conf.min_temp) * eq
                - conf.elevation_temp_drop * above_sea;
        }
    }

    Ok(())
}

#[test]
fn world_gen_test() -> Result<()> {
    let conf = WorldGenConf {
        elevation_range: -30.0..210.0,
        perlin_noise_seed: 1,
        perlin_octaves: 4,
        perlin_frequency: 0.05,
        perlin_persistence: 0.5,
        max_temp: 32.0,
        min_temp: -20.0,
        elevation_temp_drop: 20.0,
    };
    let world = World::new(30, 15, &conf)?;

    for c in &world.cells {
        let e = c.terrain.elevation as f64;
        assert!(
            conf.elevation_range.start <= e && e <= conf.elevation_range.end,
            "elevation {e} outside {:?}",
            conf.elevation_range
        );
    }

    let mid = world.height / 2;
    let hottest = |v: u32| {
        world
            .cells
            .iter()
            .filter(|c| c.v == v)
            .map(|c| c.temp)
            .fold(f32::MIN, f32::max)
    };
    assert!(hottest(mid) > hottest(0), "equator not warmer than pole");

    Ok(())
}
