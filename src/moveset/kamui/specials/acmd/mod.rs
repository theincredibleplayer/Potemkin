use super::*;

mod potbuster;
mod roman_cancel;
mod specialairhi;
mod specialhi;
mod speciallw;
mod specialn;
mod specials;

pub fn install(agent: &mut Agent) {
    potbuster::install(agent);    
    roman_cancel::install(agent);
    specialairhi::install(agent);    
    specialhi::install(agent);    
    speciallw::install(agent);    
    specialn::install(agent);    
    specials::install(agent);
}