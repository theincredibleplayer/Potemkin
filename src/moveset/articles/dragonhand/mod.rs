use super::*;


pub unsafe extern "C" fn special_lw_hit(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.7);
    }
}

pub unsafe extern "C" fn effect_special_lw_hit(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_lw_hit(agent: &mut L2CAgentBase) {} 




pub fn install(agent: &mut Agent) {
    agent.game_acmd("game_dhspecialnendmax", special_lw_hit, Priority::Low);
    agent.effect_acmd("effect_dhspecialnendmax", effect_special_lw_hit, Priority::Low);
    agent.sound_acmd("sound_dhspecialnendmax", sound_special_lw_hit, Priority::Low);

}