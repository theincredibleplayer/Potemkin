use super::*;

mod landing;
mod aerials;
mod air_throw;
mod items;
mod normals;
mod smash_attacks;
mod taunt;
mod throw;
mod tilts;
mod win;



pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    air_throw::install(agent);
    items::install(agent);
    normals::install(agent);
    smash_attacks::install(agent);
    taunt::install(agent);
    throw::install(agent);
    tilts::install(agent);
    landing::install(agent);
    win::install(agent);
}