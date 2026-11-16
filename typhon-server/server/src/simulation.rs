use anyhow::Result;

use crate::terrain::World;

#[derive(Debug)]
pub struct Simulation {
    year: u32,
    world: crate::terrain::World,
}

pub async fn simulation() -> Result<()> {
    let world = World::new(20, 10)?;
    println!("{}", world);
    Ok(())
}
