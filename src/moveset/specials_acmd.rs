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

pub unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_hammerfall_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_00"));
    }
}

pub unsafe extern "C" fn game_specialsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("hip"), 15.0, -15.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
}

pub unsafe extern "C" fn effect_specialsloop(agent: &mut L2CAgentBase) {
    for n in 1..99{
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_fdb_fire"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    wait(agent.lua_state_agent, 30.0);
    }
}
pub unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
}
pub unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.75);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 35, 80, 0, 50, 16, 0.0, 14.0, 6.0, Some(0.0), Some(-2.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("dolly_buster_dash_ground"), false, true);
    }
}

pub unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_hammerfall_break_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_a"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_b"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_hammerfall_c"));
    }
}

pub unsafe extern "C" fn game_specialsmff(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("handr"),15.0,300,80,0,40,6.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,1,0,Hash40::new("handl"),15.0,300,80,0,40,6.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,2,0,Hash40::new("handr"),15.0,75,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,3,0,Hash40::new("handl"),15.0,75,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub unsafe extern "C" fn sound_specialsmff(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_megafist_X".to_string(),2,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}