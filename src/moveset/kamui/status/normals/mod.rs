use super::*;



mod air_throw;
mod tilts;


pub fn install(agent: &mut Agent) {
    air_throw::install(agent);
    tilts::install(agent);
}