use super::*;

mod specialhi;
mod speciallw;



pub fn install(agent: &mut Agent) {
    specialhi::install(agent);
    speciallw::install(agent);
}