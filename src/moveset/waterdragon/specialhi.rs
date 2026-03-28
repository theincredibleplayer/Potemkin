use super::*;


pub unsafe extern "C" fn special_dragon_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
    }
}

pub unsafe extern "C" fn special_hi_start_dragon(agent: &mut L2CAgentBase) {
    if !camera_checks(agent){
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
    }
}

pub unsafe extern "C" fn special_dragon_hi_loop(agent: &mut L2CAgentBase) {
    if !camera_checks(agent){
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
    }
}
pub unsafe extern "C" fn special_dragon_hi_release(agent: &mut L2CAgentBase) {
    if !camera_checks(agent){
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
    }
}

pub unsafe extern "C" fn entry_lr_throw_hi(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_entry_lr_throw_hi(_agent: &mut L2CAgentBase) {}



pub fn install(agent: &mut Agent) {
    agent.game_acmd("game_specialhi", special_dragon_hi, Priority::Low);
    agent.game_acmd("game_entryl", special_hi_start_dragon, Priority::Low);
    agent.game_acmd("game_entryr", special_dragon_hi_loop, Priority::Low);
    agent.game_acmd("game_win3", special_dragon_hi_release, Priority::Low);
    agent.game_acmd("game_entryl", entry_lr_throw_hi, Priority::Low);
    agent.game_acmd("game_throwhi", entry_lr_throw_hi, Priority::Low);
    agent.effect_acmd("effect_entryl", effect_entry_lr_throw_hi, Priority::Low);
    agent.effect_acmd("effect_entryr", effect_entry_lr_throw_hi, Priority::Low);
    agent.effect_acmd("effect_throwhi", effect_entry_lr_throw_hi, Priority::Low);

}