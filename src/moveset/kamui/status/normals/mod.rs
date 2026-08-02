use super::*;



mod air_throw;
mod tilts;
mod lose;
mod item_heavy;


pub fn install(agent: &mut Agent) {
    air_throw::install(agent);
    tilts::install(agent);
    lose::install(agent);
    item_heavy::install(agent);
}