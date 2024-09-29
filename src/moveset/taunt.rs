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
    UiManager::set_bar_out(true);
    WorkModule::inc_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_TEST);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_APPEAL)(fighter)
}

pub unsafe extern "C" fn appeal_srl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        UiManager::set_bar(true);
        
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
        //ui_disable(0x1100b8cc00, 0x4f82860);
    }
}

pub unsafe extern "C" fn effect_appeal_srl(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn sound_appeal_srl(_agent: &mut L2CAgentBase) {} 