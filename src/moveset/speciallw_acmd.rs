use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::macros::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::{Vector2f,Vector3f};
use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash_script::macros;
use std::arch::asm;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;
use crate::moveset::*;

pub unsafe extern "C" fn special_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_lw_hit"),false,-1.0);
        }else{
            ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_air_lw"),false,-1.0);
        }
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
        
        macros::ATTACK(agent,0,0,Hash40::new("top"),10.0,60,100,0,0,8.0, 0.0, 10.0, 20.0, Some(0.0), Some(10.0), Some(25.0),3.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_NONE);
        let pos = Vector3f{x:9.0, y:-6.0, z:2.0}; //z is left ad right, y is up and down, x is in and out
        let rot = Vector3f{x:0.0, y:0.0, z:0.0};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_garuda_blast"), Hash40::new("arml"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:2.0,y:1.5,z:2.0} );
        let pos2 = Vector3f{x:11.0, y:-6.0, z:0.0}; //z is left ad right, y is up and down, x is in and out
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("arml"), &pos2, &rot, 1.2, true, 0, 0, 0, 0, 0, true, true) as u32;
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);


        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
    }
    sv_animcmd::wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::FT_MOTION_RATE(agent, 0.7);
    }
}

pub unsafe extern "C" fn effect_special_lw(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_lw(agent: &mut L2CAgentBase) {
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_garuda_impact_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}

pub unsafe extern "C" fn special_lw_hit(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.7);
    }
}

pub unsafe extern "C" fn effect_special_lw_hit(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_lw_hit(agent: &mut L2CAgentBase) {} 