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
use smash_script::{*, macros::*};

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;

use crate::moveset::*;

pub unsafe extern "C" fn game_romancancel(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.3, 0.3, 0.3, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            macros::SLOW_OPPONENT(agent, 400.0, 29.0);
            let sound = sound_selector(agent,"vc_pot_red_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){
            macros::SLOW_OPPONENT(agent, 400.0, 60.0);
            let sound = sound_selector(agent,"vc_pot_purple_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            macros::SLOW_OPPONENT(agent, 400.0, 29.0);
            let sound = sound_selector(agent,"vc_pot_yellow_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){
            macros::SLOW_OPPONENT(agent, 7.0, 60.0);
            let sound = sound_selector(agent,"vc_pot_blue_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }
    }

    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            macros::EFFECT(agent, Hash40::new("kamui_red_rc"), Hash40::new("top"),-2, 18, -10, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n05b"));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){
            macros::EFFECT(agent, Hash40::new("kamui_purple_rc"), Hash40::new("top"), -2, 18, -10, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n05"));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            macros::EFFECT(agent, Hash40::new("kamui_yellow_rc"), Hash40::new("top"), -2, 18, -10, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n06"));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){
            macros::EFFECT(agent, Hash40::new("kamui_blue_rc"), Hash40::new("top"), -2, 18, -10, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n04"));
        }
        //macros::EFFECT(agent, Hash40::new("kamui_red_rc"), Hash40::new("bust"), 0, -5, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, false);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_red_rc"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 6, true);
    }

    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE);
        }
    }

    sv_animcmd::frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE);
        }
    }

    sv_animcmd::frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
        
            macros::ATTACK(agent,0,0,Hash40::new("top"),5.0,65,70,20,70,20.0,-2.0,16.0,0.0,None,None,None,1.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
            //macros::EFFECT(agent, Hash40::new("kamui_red_rc_blast"), Hash40::new("top"), -2, 16, -10, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, false);        
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_red_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){

            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_purple_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);

            //macros::EFFECT(agent, Hash40::new("kamui_purple_rc_blast"), Hash40::new("top"), -2, 16, -10, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, false);        
        
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){

            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_yellow_rc_blast"), Hash40::new("top"),  10, 16, -2, 0, 0, 0, 5, true);

            macros::SEARCH(agent, 0, 0, Hash40::new("top"), 30.0, -2.0, 16.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
            //macros::EFFECT(agent, Hash40::new("kamui_yellow_rc_blast"), Hash40::new("top"),-2, 16, -10, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, false);        
        
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){

            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_blue_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);

            //macros::EFFECT(agent, Hash40::new("kamui_blue_rc_blast"), Hash40::new("top"), -2, 16, -10, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, false);        
        
        }
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_rc_burst"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 6, true);
        //
        //if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
        //    macros::ATTACK(agent,0,0,Hash40::new("bust"),5.0,65,70,20,70,20.0,0.0,0.0,0.0,None,None,None,1.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        //}
    }

    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            AttackModule::clear_all(agent.module_accessor);
        }
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    }

    sv_animcmd::frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 20);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC);

    }
}