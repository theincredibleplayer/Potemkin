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

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_lw"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_lw"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_lw"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if ArticleModule::is_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub unsafe extern "C" fn special_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_lw_hit"),false,-1.0);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),10.0,60,100,0,0,15.0,0.0,10.0,20.0,None,None,None,3.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_saving"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_NONE);
        AttackModule::set_attack_level(agent.module_accessor,0,*FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_no_finish_camera(agent.module_accessor,0,true,false);
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 6.5, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
    }
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_special_lw(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_lw(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn special_lw_hit(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_special_lw_hit(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_lw_hit(_agent: &mut L2CAgentBase) {} 