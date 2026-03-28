use super::*;


mod specialn;


pub fn install(agent: &mut Agent) {
    specialn::install(agent);
}