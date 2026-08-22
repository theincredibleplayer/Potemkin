use super::*;

mod auto_turn_squat;
mod auto_turn_air;
mod auto_turn;
mod fall;
mod landing;
mod pass;
mod turn;
mod wait;
mod walk_back;
mod walk;
mod squat;
pub mod status_end;

use auto_turn_squat::*;
use auto_turn_air::*;
use auto_turn::*;
use fall::*;
use landing::*;
use pass::*;
use turn::*;
use wait::*;
use walk_back::*;
use walk::*;
use squat::*;
pub use status_end::*;


pub fn install(agent: &mut Agent) {
    auto_turn_squat::install(agent);
    auto_turn_air::install(agent);
    auto_turn::install(agent);
    fall::install(agent);
    landing::install(agent);
    pass::install(agent);
    turn::install(agent);
    wait::install(agent);
    walk_back::install(agent);
    walk::install(agent);
    squat::install(agent);
}