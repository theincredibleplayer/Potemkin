use super::*;

mod normals;
mod specials;



pub fn install(agent: &mut Agent) {
    normals::install(agent);
    specials::install(agent);
}