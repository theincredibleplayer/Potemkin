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

pub unsafe extern "C" fn throw_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 80, 140, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }

    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 80, 80, 0, 50,6.0, 0.0, 20.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);


        
        //smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(agent_cutin_manager, Vector3f{x: 3.0, y: 6.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 158.0);
    if macros::is_excute(agent){
        macros::CHECK_FINISH_CAMERA(agent, -5, 18);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 5.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 159.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }

}

pub unsafe extern "C" fn effect_throw_lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        for _ in 0..2 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("haver"), 0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("havel"), -0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {

        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 129.0);
    if macros::is_excute(agent) {
        //macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, false);
        //macros::EFFECT(agent, Hash40::new("demon_heavens_impact"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        //macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        //macros::EFFECT(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        //EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:3.0,y:1.0,z:3.0} );
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, true);

        macros::LAST_EFFECT_SET_RATE(agent, 0.5);

        macros::EFFECT(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:2.0,y:1.0,z:2.0} );
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

        //macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }

    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("kamui_buster_fire"), false, true);
    }
}

pub unsafe extern "C" fn sound_throw_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),3,true,false,0);
        //macros::PLAY_SE(agent, Hash40::new_raw(sound));
        SoundModule::play_se(agent.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n03"));
    }
    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),3,true,true,2);
        SoundModule::play_se(agent.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
        //macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

pub unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }

    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::REQ_MOTION_CAMERA(agent, Hash40::new("d04potemkinbuster.nuanmb"), false);
    }
    frame(agent.lua_state_agent, 103.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 150.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
}



pub unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 280, 80, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 10.0, 280, 110, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 17, 11);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {

}

pub unsafe extern "C" fn expression_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // check what this does
        //AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("arml"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        //macros::FT_ATTACK_ABS_CAMERA_QUAKE(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
        PostureModule::update_rot_y_lr(agent.module_accessor);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 280, 80, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 10.0, 280, 110, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 17, 11);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}