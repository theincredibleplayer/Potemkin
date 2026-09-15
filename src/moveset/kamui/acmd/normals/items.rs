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


unsafe extern "C" fn game_screw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..10 {
        let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
        L2CFighterAnimcmdGameCommon::game_ScrewCommon(animcmd);
        if macros::is_excute(agent) {
            attack!(agent, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0, 5, 12);
            attack!(agent, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0, 5, -4);
            attack!(agent, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0, 18, 12);
            attack!(agent, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0, 18, -4);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

unsafe extern "C" fn effect_screw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_screw"), Hash40::new("rot"), 0, 1, 3, 0, 0, 0, 2.0, true);
    }
}






unsafe extern "C" fn game_itemlightthrowf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 34, 16, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemlightthrowb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
        PostureModule::update_rot_y_lr(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 34, 16, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemlightthrowhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 48, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn game_itemlightthrowlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 3, 1, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
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

    agent.game_acmd         ("game_screw",                  game_screw,                 Priority::Low);
    agent.game_acmd         ("game_screwair",               game_screw,                 Priority::Low);
    agent.effect_acmd       ("effect_screw",                effect_screw,               Priority::Low);
    agent.effect_acmd       ("effect_screwair",             effect_screw,               Priority::Low);

    agent.game_acmd         ("game_itemlightthrowf",        game_itemlightthrowf,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowf4",       game_itemlightthrowf,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowb",        game_itemlightthrowb,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowb4",       game_itemlightthrowb,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowhi",       game_itemlightthrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowhi4",      game_itemlightthrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowlw",       game_itemlightthrowlw,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowlw4",      game_itemlightthrowlw,      Priority::Low);


    agent.game_acmd         ("game_itemlightthrowairf",        game_itemlightthrowf,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairf4",       game_itemlightthrowf,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairb",        game_itemlightthrowb,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairb4",       game_itemlightthrowb,       Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairhi",       game_itemlightthrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairhi4",      game_itemlightthrowhi,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairlw",       game_itemlightthrowlw,      Priority::Low);
    agent.game_acmd         ("game_itemlightthrowairlw4",      game_itemlightthrowlw,      Priority::Low);



}