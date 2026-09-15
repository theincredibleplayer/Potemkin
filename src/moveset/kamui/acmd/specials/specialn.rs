use super::*;



unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_fdb_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}

unsafe extern "C" fn expression_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }

}



unsafe extern "C" fn game_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, 2);
    }
}

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 5.0);
}

unsafe extern "C" fn sound_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n01"));
    }
}

unsafe extern "C" fn expression_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}




unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED){
            shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, 2);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, 2);
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2){
            ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
            potemkin_mirror_handler(agent.module_accessor);
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_n_fire"), false, -1.0);
            let plane = ArticleModule::get_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
            let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            LinkModule::remove_model_constraint(plane_boma,true);
            LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_n_fire"), false, -1.0);

            macros::ATTACK(agent,0,0,Hash40::new("top"),10.0,60,100,0,40,8.0,0.0,12.0,11.0,Some(0.0),Some(12.0),Some(24.0),3.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_NONE);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2);
        }
        if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_GROUND{
            macros::SET_SPEED_EX(agent, -0.9, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::FT_MOTION_RATE(agent, 3.0);
    }
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        if macros::is_excute(agent) {
            if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_AIR{
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, -10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);    
            }
        }
    }
}
unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n01"));
    }
}

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_N_FLAG_MOTION_END2);
    }
}






pub fn install(agent: &mut Agent) {
    //.game_acmd("game_finalairlend", game_finalend, Priority::Low)
    agent.sound_acmd         ("sound_specialnstart",                    sound_specialnstart,                Priority::Low);
    agent.effect_acmd        ("effect_specialnstart",                   effect_specialnstart,               Priority::Low);
    agent.expression_acmd    ("expression_specialnstart",               expression_specialnstart,           Priority::Low);

    agent.game_acmd          ("game_specialnhold",                      game_specialnhold,                  Priority::Low);
    agent.sound_acmd         ("sound_specialnhold",                     sound_specialnhold,                 Priority::Low);
    agent.effect_acmd        ("effect_specialnhold",                    effect_specialnhold,                Priority::Low);
    agent.expression_acmd    ("expression_specialnhold",                expression_specialnhold,            Priority::Low);

    agent.game_acmd          ("game_specialnfire",                      game_specialnfire,                  Priority::Low);
    agent.sound_acmd         ("sound_specialnfire",                     sound_specialnfire,                 Priority::Low);
    agent.effect_acmd        ("effect_specialnfire",                    effect_specialnfire,                Priority::Low);
    agent.expression_acmd    ("sound_specialnfire",                     sound_specialnfire,                 Priority::Low);

    agent.sound_acmd         ("sound_specialairnstart",                 sound_specialnstart,                Priority::Low);
    agent.effect_acmd        ("effect_specialairnstart",                effect_specialnstart,               Priority::Low);
    agent.expression_acmd    ("expression_specialairnstart",            expression_specialnstart,           Priority::Low);

    agent.game_acmd          ("game_specialairnhold",                   game_specialnhold,                  Priority::Low);
    agent.sound_acmd         ("sound_specialairnhold",                  sound_specialnhold,                 Priority::Low);
    agent.effect_acmd        ("effect_specialairnhold",                 effect_specialnhold,                Priority::Low);
    agent.expression_acmd    ("expression_specialairnhold",             expression_specialnhold,            Priority::Low);

    agent.game_acmd          ("game_specialairnfire",                   game_specialnfire,                  Priority::Low);
    agent.sound_acmd         ("sound_specialairnfire",                  sound_specialnfire,                 Priority::Low);
    agent.effect_acmd        ("effect_specialairnfire",                 effect_specialnfire,                Priority::Low);
    agent.expression_acmd    ("sound_specialairnfire",                  sound_specialnfire,                 Priority::Low);
}