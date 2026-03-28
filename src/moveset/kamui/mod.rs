use super::*;

mod finalsmash;
mod specials;
mod normals;
mod autoturn;




pub fn install(agent: &mut Agent) {
    specials::install(agent);
    normals::install(agent);
    autoturn::install(agent);
    finalsmash::install(agent);
}