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
        //macros::EFFECT(agent, Hash40::new("sys_genesis_bg"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
    if macros::is_excute(agent) {
        ModelModule::set_scale_z(agent.module_accessor,1.0);
        PostureModule::update_rot_y_lr(agent.module_accessor);
        UiManager::set_bar_off();
        set_vis_hud(true);
        //set_stage_visibility(agent.module_accessor, false);
        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
        let enemy = sv_battle_object::module_accessor(capture_id as u32);
        let lr = LinkModule::get_parent_lr(enemy,*LINK_NO_CAPTURE);
        PostureModule::set_lr(enemy, lr);
        PostureModule::update_rot_y_lr(enemy);
    }


    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 60, 0, 170, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 20.0, 60, 0, 170, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        //HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }


    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 35, 50, 0, 50, 25, 0.0, 10.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, true);
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        //SlowModule::set_whole(enemy, 4, 30);
        //MotionModule::change_motion(enemy,Hash40::new("down_wait_d"),0.0,1.0,false,0.0,false,false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
        let enemy = sv_battle_object::module_accessor(capture_id as u32);
        let z_scale = WorkModule::get_float(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_POTEMKIN_Z_SCALE);
        ModelModule::set_scale_z(agent.module_accessor,z_scale);

        if DamageModule::damage(enemy, 0) > 100.0 {
            macros::FT_MOTION_RATE(agent, 2.0);
            EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            if capture_id != 0x50000000 {
                AttackModule::hit_absolute_joint(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
                CameraModule::reset_all(agent.module_accessor); 
                CAM_ZOOM_OUT_FINAL(agent);
                WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
                wait(agent.lua_state_agent, 1.0);
                if macros::is_excute(agent) {
                    let lr = PostureModule::lr(enemy);
                    TurnModule::set_turn(enemy, Hash40::new("back_damage"), lr, false, false, true);
                    PostureModule::reverse_lr(enemy);
                    PostureModule::update_rot_y_lr(enemy);

                }
            }
            for n in 1..21{
                wait(agent.lua_state_agent, 1.0);
                if n<=3{
                    EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_RATE(agent, 3.0);
                }
                SlowModule::set_whole(agent.module_accessor, 3, 100);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                let common = get_fighter_common_from_accessor(&mut* enemy);
                //set_stage_visibility(agent.module_accessor, false);               ////////////////////////////////problem here
                macros::QUAKE(common, *CAMERA_QUAKE_KIND_XL);
                SlowModule::clear_whole(agent.module_accessor);
                
                StopModule::set_hit_stop_frame(enemy, 20, true);
                let handle = EffectModule::req_screen(enemy, Hash40::new("bg_finishhit"), false, true, true);
                EffectModule::set_billboard(enemy, handle as u32, true);
                EffectModule::set_disable_render_offset_last(enemy);
                SoundModule::play_se(enemy, Hash40::new("se_common_finishhit"), false, false, false, false, enSEType(0));
                let scale = PostureModule::scale(enemy);
                CameraModule::reset_all(enemy);            
                macros::CAM_ZOOM_IN_arg6(
                    common,
                    /*frames to zoom in totally*/12.0,
                    /*according to calude the object id?*/0.0,
                    /*the amt of zoom needed, higher the number lower the zoom*/1.3*scale,
                    /*y_rot*/0.0, // up and down
                    /*x_rot*/0.0, //left and right
                    true
                );
                
                SlowModule::set_whole(enemy, 9, 40);
                EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("sys_bg_finishhit"), false, false);
                smash::app::sv_animcmd::EFFECT_OFF_KIND(agent.lua_state_agent);
                agent.pop_lua_stack(1);
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("sys_bg_black"), false, false);
                smash::app::sv_animcmd::EFFECT_OFF_KIND(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
            wait(agent.lua_state_agent, 4.0);
            //frame(agent.lua_state_agent, 152.0);
            if macros::is_excute(agent) {
                let common = get_fighter_common_from_accessor(&mut* enemy);
                //set_stage_visibility(agent.module_accessor, true);
                macros::FT_MOTION_RATE(agent, 1.0);
                macros::CAM_ZOOM_OUT(common);
                StatusModule::change_status_request_from_script(enemy, FIGHTER_STATUS_KIND_DEAD.into(), false.into());
            }
            
        }else{
            if capture_id != 0x50000000 {
                AttackModule::hit_absolute_joint(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
                CameraModule::reset_all(agent.module_accessor); 
                CAM_ZOOM_OUT_FINAL(agent);
                WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
                wait(agent.lua_state_agent, 1.0);
                if macros::is_excute(agent) {
                    let lr = PostureModule::lr(enemy);
                    TurnModule::set_turn(enemy, Hash40::new("back_damage"), lr, false, false, true);
                    PostureModule::reverse_lr(enemy);
                    PostureModule::update_rot_y_lr(enemy);
                }    
            }
            
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
    if macros::is_excute(agent) {
        for _ in 0..2 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("haver"), 0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("havel"), -0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }


    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        UiManager::set_explosion_1_on();
        macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, 7, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, -5, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        //macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 0, -20, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, false);
        //macros::LAST_EFFECT_SET_RATE(agent, 0.3);
        
        macros::EFFECT(agent, Hash40::new("kamui_floor_crack_buster"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.2);

        macros::EFFECT(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:5.0,y:3.0,z:5.0} );
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }

    for n in 1..5{
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            //macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, true);
            //macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        }
    }
    
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("kamui_buster_fire"), false, true);
    }
}



pub unsafe extern "C" fn game_heavenlystart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        JostleModule::set_status(agent.module_accessor, false);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {

        let lr = PostureModule::lr(agent.module_accessor);
        macros::PLAY_SE(agent, Hash40::new("se_kamui_final04"));
        macros::FT_MOTION_RATE(agent, 6.0);
        macros::SLOW_OPPONENT(agent, 400.0, 30.0);
        CameraModule::reset_all(agent.module_accessor);            
        let lr = PostureModule::lr(agent.module_accessor);
        macros::CAM_ZOOM_IN_arg5(
            agent,
            /*frames to zoom in totally*/5.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/1.4,
            /*y_rot*/0.35,
            /*x_rot*/-0.35*lr,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/15.0 * lr,
        /*y*/15.0,
        /*z*/0.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_genesis_bg"), false, true);
        macros::FT_MOTION_RATE(agent, 1.0);
        SlowModule::clear_whole(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
        macros::CATCH(agent, 0, Hash40::new("hip"), 20.0,0.0,0.0,0.0, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_A);
        macros::CAM_ZOOM_OUT(agent);
    }
}



pub unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        CameraModule::reset_all(agent.module_accessor);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 70.0);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04final.nuanmb"), false, false);
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        //SlowModule::clear_whole(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH);
    }
}

pub unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        let eff = EffectModule::req(agent.module_accessor, Hash40::new("sys_bg_vortex"),  &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, 0, 0, false,0);
        EffectModule::set_rgb(agent.module_accessor, eff as u32, 0.0,0.0,0.3);
        macros::EFFECT(agent, Hash40::new("sys_bg_lightning"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.3, 0.3, 0.3, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);

        //macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }//
    frame(agent.lua_state_agent, 20.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
        if macros::is_excute(agent) {
            for boi in 1..12 {
                //macros::EFFECT(agent, Hash40::new("sys_dragoon_a"), Hash40::new("top"), 0, 12, 0, sv_math::rand(hash40("fighter"),180), 90, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    else{
        if macros::is_excute(agent) {
            for boi in 1..12 {
              //macros::EFFECT(agent, Hash40::new("sys_dragoon_a"), Hash40::new("top"), 0, 12, 0, sv_math::rand(hash40("fighter"),180), 90, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        for n in 1..9{
            let random = smash::app::sv_math::rand(hash40("fighter"), 2);
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handl"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("g_chest_gear_r"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("g_chest_gear_l"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }

            wait(agent.lua_state_agent, 5.0);
        }
    }


    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bg_lightning"), false, true);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bg_vortex"), false, true);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bg_black"), false, true);
        macros::CANCEL_FILL_SCREEN(agent, 0, 20);
    }
}

pub unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kamui_final01"));
        macros::PLAY_SE(agent, Hash40::new("se_kamui_final03"));
    }
}


pub unsafe extern "C" fn game_finalvisualscene01(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        CameraModule::zoom_out(agent.module_accessor, 0);
    }
}
pub unsafe extern "C" fn sound_finalvisualscene01(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn game_fakefinalvisualscene01(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        set_vis_hud(false);
        UiManager::set_bar_on();
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
        
        let enemy_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
        let enemy = sv_battle_object::module_accessor(enemy_id as u32);
        macros::SLOW_OPPONENT(agent, 400.0, 400.0);
        SlowModule::clear(enemy);
    }
    frame(agent.lua_state_agent, 145.0);
    if macros::is_excute(agent) {
        UiManager::set_jetpack_on();
    }
    frame(agent.lua_state_agent, 229.0);
    if macros::is_excute(agent) {
        for _ in 0..6 {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("hip"), -10, -2, -15, -90, 90, 0, 0.8, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.8);
                macros::EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("hip"), -10, -2, 15, -90, 90, 0, 0.8, true);
                macros::LAST_EFFECT_SET_RATE(agent, 1.0);
            }
            wait(agent.lua_state_agent, 7.0);
        }
    }
}

pub unsafe extern "C" fn sound_fakefinalvisualscene01(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent) {
            let sound = sound_selector(agent,"vc_pot_heavenly_X_X".to_string(),3,true,false,1);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
            macros::PLAY_SE(agent, Hash40::new("se_kamui_final01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            let sound = sound_selector(agent,"vc_pot_heavenly_X_X".to_string(),3,true,true,2);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }
        frame(agent.lua_state_agent, 240.0);
        if macros::is_excute(agent) {
            let sound = sound_selector(agent,"vc_pot_heavenly_X_X".to_string(),3,true,true,3);
            macros::PLAY_SE(agent, Hash40::new_raw(sound));
        }
}


pub unsafe extern "C" fn game_finalvisualscene02(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        
        //JostleModule::set_status(agent.module_accessor, false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_02"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("final_visual_scene_02"), false, -1.0);
        let plane = ArticleModule::get_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true); 
        macros::FT_SET_FINAL_FEAR_FACE(agent, 150); 
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        //VisibilityModule::set_whole(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
    }
}

pub unsafe extern "C" fn sound_finalvisualscene02(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
    let module_accessor = agent.module_accessor;
    let enemy_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    let enemy_kind = sv_battle_object::kind(enemy_id as u32);
    let name = LOWERCASE_FIGHTER_NAMES.get(enemy_kind as usize).unwrap();
    let result = ["vc_",name,"_damage_twinkle"].join("");
    let scream_handle = SoundModule::play_se(enemy, Hash40::new(result.as_str()), true, false, false, false, enSEType(0));
    SoundModule::set_se_vol(enemy, scream_handle as i32, 2.0, 0);
    //for zarek's pacman mod
    let name = LOWERCASE_FIGHTER_NAMES.get(enemy_kind as usize).unwrap();
    let result = ["se_",name,"_damage_twinkle"].join("");
    let pac_handle = SoundModule::play_se(enemy, Hash40::new(result.as_str()), true, false, false, false, enSEType(0));
    SoundModule::set_se_vol(enemy, pac_handle as i32, 2.0, 0);


    }
    frame(agent.lua_state_agent, 130.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heavenly_X_X".to_string(),3,true,true,4);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}
pub unsafe extern "C" fn effect_finalvisualscene02(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, -10, 0, 0.0, 0.0, 0.0, 15, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        EffectModule::set_alpha_last(agent.module_accessor, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, -10, 0, 0.0, 0.0, 0.0, 10, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        EffectModule::set_alpha_last(agent.module_accessor, 0.5);
    }
}

pub unsafe extern "C" fn game_finalvisualscene03(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn sound_finalvisualscene03(agent: &mut L2CAgentBase) {
}