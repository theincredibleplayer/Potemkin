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
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 50, 150, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }

    frame(agent.lua_state_agent, 103.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 80, 80, 0, 50, 6.0, 0.0, 20.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F
        , false, 0, 0.0, 0, 
        false, false, false, 
        false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        //smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(agent_cutin_manager, Vector3f{x: 3.0, y: 6.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 104.0);
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
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("haver"), 3, 0, 4, 0.0, 0.0, 0.0, 1.25, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("havel"), -3, 0, 4, 0.0, 0.0, 0.0, 1.25, true);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("haver"), 3, 0, 4, 0.0, 0.0, 0.0, 1.25, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("havel"), -3, 0, 4, 0.0, 0.0, 0.0, 1.25, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 104.0);
    if macros::is_excute(agent) {
        //macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, false);
        //macros::EFFECT(agent, Hash40::new("demon_heavens_impact"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        //macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:3.0,y:1.0,z:3.0} );

        macros::EFFECT(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:2.0,y:1.0,z:2.0} );

        //macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    for n in 1..5{
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:3.0,y:1.0,z:3.0} );
        }
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        //EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_soccer"), false, true);
    }
}

pub unsafe extern "C" fn sound_throw_lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),2,true,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n03"));
    }
    frame(agent.lua_state_agent, 103.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),2,true,true,2);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

pub unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/49.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/0.01,
            /*y_rot*/-0.25, // up and down
            /*x_rot*/0.0 * lr, //left and right
            /*quake bool*/ true,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/0.0 * lr,
        /*y*/5.0,
        /*z*/80.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }

    for _ in 0..40 {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            let lr = PostureModule::lr(agent.module_accessor);
            agent.clear_lua_stack();
            lua_args!(agent,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/0.0 * lr,
            /*y*/0.5,
            /*z*/0.0);
            smash::app::sv_module_access::camera(agent.lua_state_agent);
        }
    }

    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
    }
    for _ in 0..19 {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            let lr = PostureModule::lr(agent.module_accessor);
            agent.clear_lua_stack();
            lua_args!(agent,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/0.0 * lr,
            /*y*/0.2,
            /*z*/0.0);
            smash::app::sv_module_access::camera(agent.lua_state_agent);
        }
    }

    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/10.0,
            /*???*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/0.01,
            /*y_rot*/0.7, // up and down
            /*x_rot*/0.0 * lr, //left and right
            /*quake bool*/ true,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/0.0 * lr,
        /*y*/4.0,
        /*z*/80.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }

    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/6.0,
            /*???*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/0.01,
            /*y_rot*/0.75, // up and down
            /*x_rot*/0.0 * lr, //left and right
            /*wuake bool*/ true,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/0.0 * lr,
        /*y*/4.0,
        /*z*/80.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/13.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/0.01,
            /*y_rot*/0.7, // up and down
            /*x_rot*/0.0 * lr, //left and right
            /*this seems to be a quake multiplier*/ true,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/0.0 * lr,
        /*y*/4.0,
        /*z*/80.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }


    frame(agent.lua_state_agent, 98.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        let lr = PostureModule::lr(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            agent,
            /*frames to zoom in totally*/4.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/0.01,
            /*y_rot*/0.1, // up and down
            /*x_rot*/0.0 * lr, //left and right
            /*this seems to be a quake multiplier*/ true,
        );
        agent.clear_lua_stack();
        lua_args!(agent,
        *MA_MSC_CMD_CAMERA_CAM_OFFSET,
        /*x*/0.0 * lr,
        /*y*/0.0,
        /*z*/70.0);
        smash::app::sv_module_access::camera(agent.lua_state_agent);
    }



    frame(agent.lua_state_agent, 103.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
}