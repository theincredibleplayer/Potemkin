use super::*;

unsafe extern "C" fn game_itemheavythrowf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 27, 22.5, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemheavythrowb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
        PostureModule::update_rot_y_lr(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 27, 22.5, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemheavythrowhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 44, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemheavythrowlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 12.5, 0.5, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}


pub fn install(agent: &mut Agent) {
    agent.game_acmd         ("game_itemheavythrowf",        game_itemheavythrowf,       Priority::Low);
    agent.game_acmd         ("game_itemheavythrowf4",       game_itemheavythrowf,       Priority::Low);
    agent.game_acmd         ("game_itemheavythrowb",        game_itemheavythrowb,       Priority::Low);
    agent.game_acmd         ("game_itemheavythrowb4",       game_itemheavythrowb,       Priority::Low);
    agent.game_acmd         ("game_itemheavythrowhi",       game_itemheavythrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemheavythrowhi4",      game_itemheavythrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemheavythrowlw",       game_itemheavythrowlw,      Priority::Low);
    agent.game_acmd         ("game_itemheavythrowlw4",      game_itemheavythrowlw,      Priority::Low);


}