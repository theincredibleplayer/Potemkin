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
use skyline::hooks::{Region,getRegionAddress};
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use nnsdk::ui2d::*;
use skyline::libc::c_void;


/*#[skyline::from_offset(0xb5f380)]
unsafe extern "C" fn kamui_link_event_original();

unsafe extern "C" fn kamui_link_event(vtable: u64, fighter: &mut Fighter) {
    println!("link event hook successful");
    kamui_init_original();
}*/

// arg 2 as 0 hides the stage and as 1 unhides it
#[skyline::from_offset(0x159fb20)]
pub unsafe extern "C" fn set_stage_visibility(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: bool);

#[skyline::from_offset(0x69a6e0)]
pub unsafe extern "C" fn get_opponent_lr(module_accessor: *mut smash::app::BattleObjectModuleAccessor, is_damage: bool, status_kind: i32, param_always_0: i32) -> f32;

#[skyline::from_offset(0x69ad00)]
pub unsafe extern "C" fn set_lr_float(lr_float: f32, module_accessor: *mut smash::app::BattleObjectModuleAccessor, status_kind: i32, param_always_0: i32) -> u64;
/*
#[skyline::from_offset(0x6955b0)]
pub unsafe extern "C" fn internal_final_cam(module: CameraModule, status_kind: Hash40, param3: i32) -> f32;
*/


#[skyline::hook(offset = 0x69a728, inline)]
unsafe extern "C" fn autoturn_situation_check_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
    let fighter_kind = utility::get_kind(&mut *(module_accessor));
    if fighter_kind == *FIGHTER_KIND_KAMUI {
        ctx.registers[0].set_x(0);
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN3app16kiiladarzmanager16stop_stage_soundEv"]
    pub fn stop_stage_sound();
}
extern "C" {
    #[link_name = "\u{1}_ZN3app13kozukatasight14set_bgm_volumeEfj"]
    pub fn set_bgm_volume(param_1: f32, param_2: i32); //param 1 is probably volume and paramm 2 is probably fade in frames
}
//squid sisters shit
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu7end_bgmEii"]
    pub fn end_bgm(param_1:i32 ,param_2:i32); // param 2 is fade in frame but no clue what param 1 is
}
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu12get_bgm_kindEv"]
    pub fn get_bgm_kind();
}
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu12get_warp_posEv"]
    pub fn get_warp_pos();
}
//sv_information
extern "C" {
    #[link_name = "\u{1}_ZN3app14sv_information13is_flat_stageEv"]
    pub fn is_flat_stage() -> bool;
}