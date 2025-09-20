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
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_hi"),false,-1.0);    
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0,0.0,34.5,20.5, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 15.0,0.0,34.5,20.5, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_A);
    }
    sv_animcmd::wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

pub unsafe extern "C" fn special_dragon_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
    }
}

pub unsafe extern "C" fn effect_special_hi(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_hi(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn special_hi_start(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_special_hi_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_02"));
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heat_extender_X_X".to_string(),2,true,false,1);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}

pub unsafe extern "C" fn special_hi_loop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
    }
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:10.0, y:15.0, z:-2.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        let pos2 = Vector3f{x:10.0, y:30.0, z:15.0}; //z is left ad right, y is up and down, x is in and out
        let rot2 = Vector3f{x:0.0, y:0.0, z:120.0*lr}; 

        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_spark"), Hash40::new("top"), &pos2, &rot2, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_attack_line_b"), Hash40::new("top"), &pos, &rot, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 18, 20, 3, -45, 0, -20, 2.5, true);
        EffectModule::set_rgb(agent.module_accessor, effect, 1.0, 0.24, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30,10,12,5.0,0.0,28.0,12.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_none"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:-10.0, y:25.0, z:9.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_extender_bullet"), Hash40::new("top"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn special_dragon_hi_loop(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn special_hi_release(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_MOTION_RATE(agent, 0.5);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 90, 0, 60, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 10.0, 361, 90, 0, 60, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE);
    }
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
    }
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:10.0, y:15.0, z:-2.0}; //z is left ad right, y is up and down, x is in and out
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        let pos2 = Vector3f{x:10.0, y:30.0, z:15.0}; 
        let rot2 = Vector3f{x:0.0, y:0.0, z:120.0*lr}; 

        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_spark"), Hash40::new("top"), &pos2, &rot2, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_attack_line_b"), Hash40::new("top"), &pos, &rot, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 18, 20, 3, -45, 0, -20, 2.5, true);
        EffectModule::set_rgb(agent.module_accessor, effect, 1.0, 0.24, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30,10,12,5.0,0.0,28.0,12.0,None,None,None
        ,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_none"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:-10.0, y:25.0, z:9.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_extender_bullet"), Hash40::new("top"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    //wait(agent.lua_state_agent, 1.0);
    //if macros::is_excute(agent) {
    //    let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
    //    let enemy = sv_battle_object::module_accessor(capture_id as u32);
    //    set_stage_visibility(agent.module_accessor, false);               ////////////////////////////////problem here
    //    StopModule::set_hit_stop_frame(enemy, 30, true);
    //    macros::PLAY_SE(agent, Hash40::new("se_common_finishhit"));
    //    EffectModule::req_follow(enemy, Hash40::new("sys_bg_finishhit"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
//
    //    //SlowModule::set_whole(agent.module_accessor, 255, 3);
    //}
    //wait(agent.lua_state_agent, 4.0);
    ////frame(agent.lua_state_agent, 152.0);
    //if macros::is_excute(agent) {
    //    let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
    //    let enemy = sv_battle_object::module_accessor(capture_id as u32);
    //    set_stage_visibility(agent.module_accessor, true);
    //    macros::FT_MOTION_RATE(agent, 1.0);
    //    StatusModule::change_status_request_from_script(enemy, FIGHTER_STATUS_KIND_DEAD.into(), false.into());
    //    WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
    //}
    sv_animcmd::frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 0, 0.0, 0.0, 0.2, 1, 1, 0.8, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 5);
    }
    
    sv_animcmd::frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        UiManager::add_tension(entry_id,18.0);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
        
        //let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        //smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.7);
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_h02"));
        macros::CAM_ZOOM_OUT(agent);
        let scale = PostureModule::scale(agent.module_accessor);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/3.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/1.3*scale,
            /*y_rot*/0.3, // up and down
            /*x_rot*/-0.6 * lr, //left and right
            true
        );
        CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/12.0 * lr,
        /*y*/17.0,
        /*z*/0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }
    
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("haver"), 0, 0, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 0, 0.0, 0.0, 0.2, 1, 1, 0.8, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 30);
    }
    sv_animcmd::wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        let scale = PostureModule::scale(agent.module_accessor);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/5.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/3.0*scale,
            /*y_rot*/0.0, // up and down
            /*x_rot*/0.0, //left and right
            true
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/12.0 * lr,
        /*y*/17.0,
        /*z*/0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        
    }

}

pub unsafe extern "C" fn sound_special_hi_release(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heat_extender_X_X".to_string(),2,true,true,2);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
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