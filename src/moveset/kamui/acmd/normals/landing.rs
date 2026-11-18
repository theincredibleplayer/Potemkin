use super::*;

unsafe extern "C" fn effect_landingairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

unsafe extern "C" fn expression_landingairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}




unsafe extern "C" fn effect_landingairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

unsafe extern "C" fn expression_landingairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}



unsafe extern "C" fn effect_landingairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

unsafe extern "C" fn expression_landingairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}



unsafe extern "C" fn game_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 110, 0, 50, 7.0, 0.0, 4.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 110, 0, 50, 7.0, 0.0, 4.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_landingairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

unsafe extern "C" fn expression_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}



unsafe extern "C" fn effect_landingairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

unsafe extern "C" fn expression_landingairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}



unsafe extern "C" fn sound_landinglight(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_btl_pot_09"));
    }
}

unsafe extern "C" fn expression_landinglight(agent: &mut L2CAgentBase) {
    let expression_common : &mut smash::lua2cpp::L2CFighterAnimcmdExpressionCommon = std::mem::transmute(&mut *agent);
    expression_common.expression_LandingLightRumble();
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}


unsafe extern "C" fn sound_landingheavy(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_btl_pot_09"));
    }
}

unsafe extern "C" fn expression_landingheavy(agent: &mut L2CAgentBase) {
    let expression_common : &mut smash::lua2cpp::L2CFighterAnimcmdExpressionCommon = std::mem::transmute(&mut *agent);
    expression_common.expression_LandingHeavyRumble();
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}


pub fn install(agent: &mut Agent) {
    agent.game_acmd          ("game_landingairlw",              game_landingairlw,              Priority::Low);
    agent.sound_acmd         ("sound_landingairlw",             sound_landingairlw,             Priority::Low);
    agent.effect_acmd        ("effect_landingairlw",            effect_landingairlw,            Priority::Low);
    agent.expression_acmd    ("expression_landingairlw",        expression_landingairlw,        Priority::Low);

    agent.sound_acmd         ("sound_landingairhi",             sound_landingairhi,             Priority::Low);
    agent.effect_acmd        ("effect_landingairhi",            effect_landingairhi,            Priority::Low);
    agent.expression_acmd    ("expression_landingairhi",        expression_landingairhi,        Priority::Low);
    
    agent.sound_acmd         ("sound_landingairf",              sound_landingairf,              Priority::Low);
    agent.effect_acmd        ("effect_landingairf",             effect_landingairf,             Priority::Low);
    agent.expression_acmd    ("expression_landingairf",         expression_landingairf,         Priority::Low);

    agent.sound_acmd         ("sound_landingairb",              sound_landingairb,              Priority::Low);
    agent.effect_acmd        ("effect_landingairb",             effect_landingairb,             Priority::Low);
    agent.expression_acmd    ("expression_landingairb",         expression_landingairb,         Priority::Low);

    agent.sound_acmd         ("sound_landingairn",              sound_landingairn,              Priority::Low);
    agent.effect_acmd        ("effect_landingairn",             effect_landingairn,             Priority::Low);
    agent.expression_acmd    ("expression_landingairn",         expression_landingairn,         Priority::Low);


    agent.sound_acmd         ("game_landingfallspecial",        sound_landingheavy,             Priority::Low);
    agent.expression_acmd    ("expression_landingfallspecial",  expression_landingheavy,        Priority::Low);

    agent.sound_acmd         ("sound_landingheavy",             sound_landingheavy,             Priority::Low);
    agent.expression_acmd    ("expression_landingheavy",        expression_landingheavy,        Priority::Low);


    agent.sound_acmd         ("sound_landinglight",             sound_landinglight,             Priority::Low);
    agent.expression_acmd    ("expression_landinglight",        expression_landinglight,        Priority::Low);

}