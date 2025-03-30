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

pub unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if ArticleModule::is_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    }
    //macros::CAM_ZOOM_OUT(fighter);
    //set_stage_visibility(fighter.module_accessor, true);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_APPEAL)(fighter)
}

pub unsafe extern "C" fn appeal_srl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        //set_stage_visibility(agent.module_accessor, false);
        //macros::CAM_ZOOM_OUT(agent);
        //let lr = PostureModule::lr(agent.module_accessor);
        //CameraModule::reset_all(agent.module_accessor);            
        //macros::CAM_ZOOM_IN_arg6(
        //    agent,
        //    /*frames to zoom in totally*/20.0,
        //    /*according to calude the object id?*/0.0,
        //    /*the amt of zoom needed, higher the number lower the zoom*/50.0,
        //    /*y_rot*/0.0, // up and down
        //    /*x_rot*/0.0 * lr, //left and right
        //    /*quake bool*/ true,
        //);
        //agent.clear_lua_stack();
        //lua_args!(agent,
        //*MA_MSC_CMD_CAMERA_CAM_OFFSET,
        ///*x*/0.0 * lr,
        ///*y*/0.0,
        ///*z*/-6000.0);
        //smash::app::sv_module_access::camera(agent.lua_state_agent);




        //smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new("prebuilt:/movie/fighter/jack/c00/final_02.h264"));
        //smash::app::lua_bind::FighterManager::start_movie(fighter_manager, 1.0);
        //macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalvisualscene02.nuanmb"), false);
        //ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        //ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("final_visual_scene_02"),false,-1.0);
        //smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_DOWN);
        //WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_TEST);
        //let main = getRegionAddress(Region::Text) as u64;
        //let ptr: u64 = main + 0x4f82860;
        //ui_disable(75174054912, ptr);
    }
}

pub unsafe extern "C" fn effect_appeal_srl(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_appeal_srl(_agent: &mut L2CAgentBase) {} 