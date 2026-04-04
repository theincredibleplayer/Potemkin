use super::*;



mod air_throw;


pub fn install(agent: &mut Agent) {
    air_throw::install(agent);
}