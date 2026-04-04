use super::*;

pub mod dragonhand;
pub mod ryusensya;
pub mod spearhand;
pub mod waterdragon;


pub fn install(agent: &mut Agent) {
    dragonhand::install(agent);
    ryusensya::install(agent);
    spearhand::install(agent);
    waterdragon::install(agent);
}