use super::*;



unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);    
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_hammerfall_dash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
}

unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_hammerfall_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_00"));
    }
}

unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5, true);
    }
}

unsafe extern "C" fn game_specialsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.0);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 6.0, 0.0, 7.0, 2.0, Some(0.0), Some(15.0), Some(2.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
}

unsafe extern "C" fn effect_specialsloop(agent: &mut L2CAgentBase) {
    loop{
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_hammerfall_dash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
            EffectModule::set_rate_last(agent.module_accessor, 0.5);
        }
        wait(agent.lua_state_agent, 30.0);
    }
}

unsafe extern "C" fn expression_specialsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0, true);
    }
}

unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.0);
        macros::FT_MOTION_RATE(agent, 0.70);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 35, 75, 0, 65, 8.0, 0.0, 14.0, 13.0, Some(0.0), Some(8.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::FT_MOTION_RATE(agent, 0.90);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_hammerfall_dash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("kamui_hammerfall_dash"), false, true);
    }
}

unsafe extern "C" fn sound_specialsattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_08"));
    }
}

unsafe extern "C" fn expression_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0, true);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0, true);
        macros::FT_MOTION_RATE(agent, 0.8);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {

        let sound = sound_selector(agent,"vc_pot_hammerfall_break_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_a"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_b"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_c"));
    }
    sv_animcmd::frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn game_specialsmff(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("handr"),15.0,300,65,0,40,4.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
        macros::ATTACK(agent,1,0,Hash40::new("handl"),15.0,300,65,0,40,4.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
        macros::ATTACK(agent,2,0,Hash40::new("handr"),10.0,75 ,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA ,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
        macros::ATTACK(agent,3,0,Hash40::new("handl"),10.0,75 ,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA ,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::FT_MOTION_RATE(agent, 1.6);
    }
}

unsafe extern "C" fn sound_specialsmff(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_megafist_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}




pub fn install(agent: &mut Agent) {
    agent.game_acmd     ("game_specials",           game_specialsstart,     Priority::Low);
    agent.sound_acmd    ("sound_specials",          sound_specialsstart,    Priority::Low);
    agent.effect_acmd   ("effect_specials",         effect_specialsstart,   Priority::Low);
    agent.expression_acmd("expression_specials",    expression_specialsstart,Priority::Low);

    agent.game_acmd     ("game_specialsslide",      game_specialsloop,      Priority::Low);
    agent.effect_acmd   ("effect_specialsslide",    effect_specialsloop,    Priority::Low);
    agent.expression_acmd("expression_specialsslide",expression_specialsloop,Priority::Low);

    agent.game_acmd     ("game_specialsattack",     game_specialsattack,    Priority::Low);
    agent.sound_acmd    ("sound_specialsattack",    sound_specialsattack,   Priority::Low);
    agent.effect_acmd   ("effect_specialsattack",   effect_specialsattack,  Priority::Low);
    agent.expression_acmd("expression_specialsattack",expression_specialsattack,Priority::Low);

    agent.sound_acmd    ("sound_specialsend",       sound_specialsend,      Priority::Low);

    agent.game_acmd     ("game_specialsmff",        game_specialsmff,       Priority::Low);
    agent.sound_acmd    ("sound_specialsmff",       sound_specialsmff,      Priority::Low);
}