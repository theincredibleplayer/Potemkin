use super::*;

mod status;
mod acmd;
mod autoturn;




pub fn install(agent: &mut Agent) {
    acmd::install(agent);
    status::install(agent);
    autoturn::install(agent);
}