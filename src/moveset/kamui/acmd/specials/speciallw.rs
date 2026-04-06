use super::*;

unsafe extern "C" fn special_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.7);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_GROUND{
            macros::SET_SPEED_EX(agent, -0.6, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_n_end_max"), false, -1.0);
        let plane = ArticleModule::get_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_n_end_max"), false, -1.0);
        
        macros::ATTACK(agent,0,0,Hash40::new("top"),10.0,60,100,0,0,8.0, 0.0, 10.0, 20.0, Some(0.0), Some(10.0), Some(25.0),3.0,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,f32::NAN,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_NONE);

    }
    sv_animcmd::wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_special_lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_AIR{
            macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, -10, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);    
        }
        let pos = Vector3f{x:9.0, y:-6.0, z:2.0}; //z is left ad right, y is up and down, x is in and out
        let rot = Vector3f{x:0.0, y:0.0, z:0.0};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_garuda_blast"), Hash40::new("arml"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:2.0,y:1.5,z:2.0} );
        let pos2 = Vector3f{x:11.0, y:-6.0, z:0.0}; //z is left ad right, y is up and down, x is in and out
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("arml"), &pos2, &rot, 1.2, true, 0, 0, 0, 0, 0, true, true) as u32;
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);

    }

}

unsafe extern "C" fn sound_special_lw(agent: &mut L2CAgentBase) {
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_10"));
        let sound = sound_selector(agent,"vc_pot_garuda_impact_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_12"));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
}

unsafe extern "C" fn expression_special_lw(agent: &mut L2CAgentBase) {
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_10"));
        let sound = sound_selector(agent,"vc_pot_garuda_impact_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_12"));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
}



pub fn install(agent: &mut Agent) {
    agent.game_acmd     ("game_speciallw",      special_lw,         Priority::Low);
    agent.effect_acmd   ("effect_speciallw",    effect_special_lw,  Priority::Low);
    agent.sound_acmd    ("sound_speciallw",     sound_special_lw,   Priority::Low);

    agent.game_acmd     ("game_specialairlw",   special_lw,         Priority::Low);
    agent.effect_acmd   ("effect_specialairlw", effect_special_lw,  Priority::Low);
    agent.sound_acmd    ("sound_specialairlw",  sound_special_lw,   Priority::Low);
}