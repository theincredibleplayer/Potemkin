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

pub unsafe extern "C" fn win_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win pre status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {

    }else{
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    return L2CValue::I32(0)

}

pub unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win main status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {
        //MotionModule::change_motion(fighter.module_accessor,Hash40::new("win_1"),0.0,1.0,false,0.0,false,false);
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("win_1"),false,-1.0);
        //CameraModule::zoom_out(fighter.module_accessor, 0);
        //macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("j02win1.nuanmb"), false, false);
    }else{
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(win_main_loop as *const () as _))
}

pub unsafe extern "C" fn win_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME) {
        //CameraModule::reset_all(fighter.module_accessor);
        //macros::REQ_MOTION_CAMERA(fighter, Hash40::new("j02win1.nuanmb"), false);
        //macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("j02win1.nuanmb"), false, false);
        //MotionModule::change_motion(fighter.module_accessor,Hash40::new("win_1"),0.0,1.0,false,0.0,false,false);
    }
    //macros::REQ_MOTION_CAMERA(fighter, Hash40::new("j02win1.nuanmb"), false);
    //macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("j02win1.nuanmb"), false, false);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    return L2CValue::I32(0)

}

pub unsafe extern "C" fn win_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win init status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {

    }else{
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    return L2CValue::I32(0)

}

pub unsafe extern "C" fn win_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win exec status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {

    }else{
        //return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    return L2CValue::I32(0)

}
pub unsafe extern "C" fn win_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win end status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {

    }else{
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    return L2CValue::I32(0)

}
pub unsafe extern "C" fn win_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("win exit status is being used");
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u64;
    let winner = smash::app::lua_bind::FighterManager::get_top_rank_player(fighter_manager, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME);
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_KAMUI
    && smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) 
    && (entry_id == winner) {

    }else{
        //return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_DEMO)(fighter);
    }
    return L2CValue::I32(0)

}




pub unsafe extern "C" fn game_win1a(agent: &mut L2CAgentBase) {

}

pub unsafe extern "C" fn sound_win1a(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_matchwin"));
    }
    frame(agent.lua_state_agent, 125.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_win_X_X".to_string(),2,true,false,1);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
    frame(agent.lua_state_agent, 220.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_win_X_X".to_string(),2,true,true,2);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}