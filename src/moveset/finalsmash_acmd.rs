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

pub unsafe extern "C" fn game_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_genesis_bg"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EffectModule::req_screen(agent.module_accessor, Hash40::new("sys_genesis_bg_criticalhit"), false, false, false);
        JostleModule::set_status(agent.module_accessor, false);
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 17.0, 13.0, 8.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
    }
}

pub unsafe extern "C" fn game_finaldashattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_genesis_bg"), false, true);
        //EffectModule::remove_screen(agent.module_accessor, Hash40::new("sys_genesis_bg_criticalhit"), -1);
        JostleModule::set_status(agent.module_accessor, false);
        macros::SLOW_OPPONENT(agent, 400.0, 7.0);
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 368, 100, 180, 0, 19.0, 13.0, 8.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 30.0, y: 50.0}, 40, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP);
    }
}

pub unsafe extern "C" fn game_finalend(agent: &mut L2CAgentBase) {
    let module_accessor = agent.module_accessor;
    let enemy_id = WorkModule::get_int(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);


    if macros::is_excute(agent){
        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_whole(enemy, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        //macros::REQ_FINAL_START_CAMERA(agent,Hash40::new("d04finalvisualscene03.nuanmb"), false);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }


    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_whole(enemy, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }


    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 35, 50, 0, 50, 25, 0.0, 10.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, true);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        WorkModule::off_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        //CameraModule::zoom_out(agent.module_accessor, 0);
        //macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_FALL, false);
        MotionModule::change_motion(enemy,Hash40::new("down_wait_d"),0.0,1.0,false,0.0,false,false);
    }


    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //KineticModule::clear_speed_all(enemy);
        AttackModule::clear_all(agent.module_accessor);
    }


    frame(agent.lua_state_agent, 108.0);
    if macros::is_excute(agent) {
        //CameraModule::zoom_out(agent.module_accessor, 0);
    }


    frame(agent.lua_state_agent, 127.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_FALL, false);
        LinkModule::remove_model_constraint(enemy,true);
        if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
            LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
        }
        if PostureModule::lr(agent.module_accessor) < 0.0{
            let pos = Vector2f{x: -15.0, y: 10.0};
            PostureModule::set_pos_2d(enemy,&pos);
            let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            PostureModule::set_rot(enemy,&rot,0);
            PostureModule::set_lr(enemy, -1.0);
        }else{
            let pos = Vector2f{x: 15.0, y: 10.0};
            PostureModule::set_pos_2d(enemy,&pos);
            let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            PostureModule::set_rot(enemy,&rot,0);
        }
      }
    if DamageModule::damage(enemy, 0) > 100.0 {


        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 20.0, 50, 0, 200, 100, 25, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            //AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 50.0, y: 15.0}, 40, false);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
            macros::FT_MOTION_RATE(agent, 2.0);
            //macros::EFFECT_FOLLOW(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
            EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        }


        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        for n in 1..21{
            if macros::is_excute(agent) {
                //SlowModule::clear_whole(agent.module_accessor);
                if n<=3{
                    EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_RATE(agent, 3.0);
                }
                SlowModule::set_whole(agent.module_accessor, 3, 100);
            wait(agent.lua_state_agent, 1.0);
            }
        }


        frame(agent.lua_state_agent, 152.0);
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 1.0);
            StatusModule::change_status_request_from_script(enemy, FIGHTER_STATUS_KIND_DEAD.into(), false.into());
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
        }


        frame(agent.lua_state_agent, 155.0);
        if macros::is_excute(agent) {
            SlowModule::clear_whole(agent.module_accessor);
        }
    } else {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 35, 100, 0, 50, 25, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }


        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
  }
  
pub unsafe extern "C" fn sound_finalend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("heavenly_1_4"));
    }

    
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_final02"));
    }
  }
  
pub unsafe extern "C" fn effect_finalend(agent: &mut L2CAgentBase) {
    for _ in 0..26 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("haver"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("havel"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
    }


    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, 7, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, -5, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 0, -20, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.3);
    }

    
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_soccer"), false, true);
    }
}
