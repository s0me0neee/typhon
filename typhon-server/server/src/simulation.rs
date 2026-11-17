use serde::Serialize;
use ts_rs::TS;

use crate::terrain::World;
use crate::ts_file;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = ts_file!())]
pub struct Status {
    pub year: u32,
}

#[derive(Debug)]
pub struct Simulation {
    pub(crate) year: u32,
    pub(crate) world: crate::terrain::World,
}

impl Simulation {
    pub fn new(world: World) -> Simulation {
        Simulation { year: 0, world }
    }
    pub fn step(&mut self) {
        self.year += 1;
    }
    pub fn status(&self) -> Status {
        Status { year: self.year }
    }
}
