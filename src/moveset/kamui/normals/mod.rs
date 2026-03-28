use super::*;

mod acmd;
mod status;



pub fn install(agent: &mut Agent) {
    acmd::install(agent);
    status::install(agent);
}