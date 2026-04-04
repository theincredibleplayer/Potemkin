use super::*;


mod aerials;
mod air_throw;
mod normals;
mod smash_attacks;
mod taunt;
mod throw;
mod tilts;



pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    air_throw::install(agent);
    normals::install(agent);
    smash_attacks::install(agent);
    taunt::install(agent);
    throw::install(agent);
    tilts::install(agent);

}