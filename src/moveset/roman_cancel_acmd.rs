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
    let start_time = WorkModule::get_int(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
    sv_animcmd::frame(agent.lua_state_agent, start_time as f32);
    if macros::is_excute(agent) {

        let stop_time = WorkModule::get_int(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
        macros::SLOW_OPPONENT(agent, 255.0, stop_time as f32);
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.3, 0.3, 0.3, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            let sound = sound_selector(agent,"vc_pot_red_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){
            let sound = sound_selector(agent,"vc_pot_purple_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            let sound = sound_selector(agent,"vc_pot_yellow_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){
            let sound = sound_selector(agent,"vc_pot_blue_rc_X".to_string(),3,false,false,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }
        //SlowModule::set_whole(agent.module_accessor, slow_mul as u8, clear_time);
    }

    sv_animcmd::wait(agent.lua_state_agent, 1.0);
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
    }

    sv_animcmd::wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE);
        }
    }

    sv_animcmd::wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE);
        }
    }

    sv_animcmd::frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 45.0, -2.0, 16.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        //let clear_time = WorkModule::get_int(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
        //let slow_mul = WorkModule::get_float(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
        //macros::SLOW_OPPONENT(agent, slow_mul, stop_time as f32);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    sv_animcmd::frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_CLOCK_EFFECT);
        //macros::SEARCH(agent, 0, 0, Hash40::new("top"), 30.0, -2.0, 16.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            macros::ATTACK(agent,0,0,Hash40::new("top"),5.0,65,70,20,70,30.0,-2.0,16.0,0.0,None,None,None,1.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_red_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_purple_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_yellow_rc_blast"), Hash40::new("top"),  10, 16, -2, 0, 0, 0, 5, true);
            //macros::SEARCH(agent, 0, 0, Hash40::new("top"), 30.0, -2.0, 16.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
        }else if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_blue_rc_blast"), Hash40::new("top"), 10, 16, -2, 0, 0, 0, 5, true);
        }

    }

    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
            AttackModule::clear_all(agent.module_accessor);
        }
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_CLOCK_EFFECT);
    }
}