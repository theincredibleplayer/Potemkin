use super::*;





pub unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::SET_SPEED_EX(agent, 1.6, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("handr"),8.0,90,95,0,35,9.0,3.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::ATTACK(agent,0,0,Hash40::new("handr"),4.0,90,70,0,35,9.0,3.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.1);
    }
}

pub unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
}

pub unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        attack_sound_randomizer(agent,"mid");
    }
    sv_animcmd::frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_14"));
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_9"));
    }
}

pub unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.7);
    }
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::ATTACK(agent,0,0,Hash40::new("footl"),10.0,75,65,0,70,4.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,1,0,Hash40::new("kneel"),8.0,75,65,0,70,4.0,3.0,0.0,0.0,Some(0.0),Some(3.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,2,0,Hash40::new("legl"),6.0,80,65,0,70,5.0,3.0,0.0,0.0,Some(0.0),Some(3.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        //macros::ATTACK(agent,3,0,Hash40::new("handl"),8.0,130,50,0,10,5.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        //macros::ATTACK(agent,4,0,Hash40::new("handr"),8.0,130,50,0,10,5.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -10, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }

}

pub unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        attack_sound_randomizer(agent,"light");
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_14"));
    }
}

pub unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}


pub unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.8);
    }
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::ATTACK(agent,0,0,Hash40::new("top"),13.0,55,89,0,50,9.0,0.0,10.0,10.0,Some(0.0),Some(19.0),Some(10.0),1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_HEAVY,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_togezoshell_speedline"), Hash40::new("top"), 10, 15, 15, -90, 0, 0, 1.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_attacks3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        attack_sound_randomizer(agent,"mid");
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
}

pub unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0, true);
    }
    sv_animcmd::frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.game_acmd          ("game_attacks3",                  game_attacks3,                  Priority::Low);
    agent.effect_acmd        ("effect_attacks3",                effect_attacks3,                Priority::Low);
    agent.sound_acmd         ("sound_attacks3",                 sound_attacks3,                 Priority::Low);
    agent.expression_acmd    ("expression_attacks3",            expression_attacks3,            Priority::Low);

    agent.game_acmd          ("game_attackhi3",                 game_attackhi3,                 Priority::Low);
    agent.effect_acmd        ("effect_attackhi3",               effect_attackhi3,               Priority::Low);
    agent.sound_acmd         ("sound_attackhi3",                sound_attackhi3,                Priority::Low);
    agent.expression_acmd    ("expression_attackhi3",           expression_attackhi3,           Priority::Low);

    agent.game_acmd          ("game_attacklw3",                 game_attacklw3,                 Priority::Low);
    agent.effect_acmd        ("effect_attacklw3",               effect_attacklw3,               Priority::Low);
    agent.sound_acmd         ("sound_attacklw3",                sound_attacklw3,                Priority::Low);
    agent.expression_acmd    ("expression_attacklw3",           expression_attacklw3,           Priority::Low);

}