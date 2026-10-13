use super::*;

// arg 2 as 0 hides the stage and as 1 unhides it
#[skyline::from_offset(SET_STAGE_VISIBILITY_FUNCTION_OFFSET.load(Ordering::Relaxed))]
pub unsafe extern "C" fn set_stage_visibility(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: bool);

#[skyline::from_offset(GET_OPPONENT_LR_FUNCTION_OFFSET.load(Ordering::Relaxed))]
pub unsafe extern "C" fn get_opponent_lr(module_accessor: *mut smash::app::BattleObjectModuleAccessor, is_damage: bool, status_kind: i32, param_always_0: i32) -> f32;

#[skyline::from_offset(SET_LR_FLOAT_FUNCTION_OFFSET.load(Ordering::Relaxed))]
pub unsafe extern "C" fn set_lr_float(lr_float: f32, module_accessor: *mut smash::app::BattleObjectModuleAccessor, status_kind: i32, param_always_0: i32) -> u64;

#[skyline::from_offset(FINAL_SMASH_CHECKS_FUNCTION_OFFSET.load(Ordering::Relaxed))]
pub unsafe extern "C" fn final_smash_checks(work_module: *const u64) -> bool;




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
extern "C" {
    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
}




