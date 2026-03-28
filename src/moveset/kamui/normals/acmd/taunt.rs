use super::*;

pub unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,ArticleOperationTarget(0));
    if smashball::is_training_mode(){
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        UiManager::add_tension_unscaled(get_ui_index_from_entry_id(entry_id),150.0);
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_APPEAL)(fighter)
}

pub unsafe extern "C" fn appeal_s(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("appeal_s"),false,-1.0);
        /*
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        let entry_id = WorkModule::get_int(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        smash::app::lua_bind::FighterManager::set_final(
            fighter_manager, 
            FighterEntryID(entry_id), 
            smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },
            0
        );*/
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {

        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_air_n_hold"), false, -1.0);
        let plane = ArticleModule::get_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("special_air_n_hold"), false, -1.0);

    }
    frame(agent.lua_state_agent, 101.0);
    if macros::is_excute(agent) {
        for n in 1..10{
            let random = smash::app::sv_math::rand(hash40("fighter"), 2);
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handl"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            wait(agent.lua_state_agent, 4.0);
        }
    }
}

pub unsafe extern "C" fn effect_appeal(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_appeal_s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_11"));
    }

    frame(agent.lua_state_agent, 104.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_taunt_hi_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));

    }
} 

pub unsafe extern "C" fn sound_appeal_hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_taunt_s_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_12"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_08"));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_09"));
    }
} 


pub unsafe extern "C" fn sound_appeal_lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_12"));
        let sound = sound_selector(agent,"vc_pot_taunt_lw_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));

    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_09"));
    }
} 





pub fn install(agent: &mut Agent) {
    agent.game_acmd     ("game_appealsr",       appeal_s,           Priority::Low);
    agent.effect_acmd   ("effect_appealsr",     effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appealsr",      sound_appeal_s,     Priority::Low);

    agent.game_acmd     ("game_appealsl",       appeal_s,           Priority::Low);
    agent.effect_acmd   ("effect_appealsl",     effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appealsl",      sound_appeal_s,     Priority::Low);

    agent.effect_acmd   ("effect_appealhir",    effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appealhir",     sound_appeal_hi,    Priority::Low);

    agent.effect_acmd   ("effect_appealhil",    effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appealhil",     sound_appeal_hi,    Priority::Low);

    agent.effect_acmd   ("effect_appeallwr",    effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appeallwr",     sound_appeal_lw,    Priority::Low);

    agent.effect_acmd   ("effect_appeallwl",    effect_appeal,      Priority::Low);
    agent.sound_acmd    ("sound_appeallwl",     sound_appeal_lw,    Priority::Low);


    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}