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


pub unsafe extern "C" fn special_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_hi"),false,-1.0);    
    }
    sv_animcmd::frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0,0.0,34.5,20.5, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 15.0,0.0,34.5,20.5, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_A);
        //macros::ATTACK(agent,0,0,Hash40::new("haver"),1.0,361,0,0,0,15.0,0.0,0.0,0.0,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_FIGHTER,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_HARISEN,*ATTACK_REGION_NONE);
        //macros::ATTACK(agent,1,0,Hash40::new("haver"),1.0,361,0,0,0,5.0,0.0,0.0,0.0,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_FIGHTER,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_HARISEN,*ATTACK_REGION_NONE);
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        //AttackModule::clear_all(agent.module_accessor);
        //WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
    }
}

pub unsafe extern "C" fn special_dragon_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
        //macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30 ,0,12,15.0,0.0,34.5,20.5,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_FIGHTER,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_HARISEN,*ATTACK_REGION_NONE);
        //macros::ATTACK(agent,1,0,Hash40::new("top"),1.0,361,30 ,0,12,5.0,0.0,34.5,20.5,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_FIGHTER,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_HARISEN,*ATTACK_REGION_NONE);
        //AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
        //AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 100.0, false);
    }
    
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        //AttackModule::clear_all(agent.module_accessor);
        //WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
    }
}

pub unsafe extern "C" fn effect_special_hi(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_hi(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn special_hi_start(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn special_hi_loop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
    }
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), -18, 20, 3, -45, 0, -20, 1.5, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        //macros::ATTACK(agent,0,0,Hash40::new("haver"),5.0,361,0,0,0,5.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn special_dragon_hi_loop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30,10,12,5.0,0.0,28.0,12.0,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 20.0, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn special_hi_release(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 361, 100, 0, 50, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 20.0, 361, 100, 0, 50, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE);
    }
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), -18, 20, 3, -45, 0, -20, 1.5, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,0,0,0,5.0,0.0,28.0,12.0,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 20.0, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

    sv_animcmd::frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RELEASE_ENEMY);
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
        //macros::ATTACK(agent,0,0,Hash40::new("haver"),20.0,361,100,0,50,15.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
        //AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_h02"));
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
        //macros::EFFECT(agent,Hash40::new("sys_bg_criticalhit"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
        //SlowModule::set_whole(agent.module_accessor,2,60);
        //macros::PLAY_SE(agent,Hash40::new("se_common_criticalhit"));
        //WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_HIT);
        //AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
        //AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }

}

pub unsafe extern "C" fn special_dragon_hi_release(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
    }
    sv_animcmd::frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
    }
}

pub unsafe extern "C" fn entry_lr_throw_hi(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_entry_lr_throw_hi(_agent: &mut L2CAgentBase) {}