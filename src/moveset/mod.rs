use smash::hash40;
use smash_script::macros::*;
use skyline::nn::ro::LookupSymbol;
use std::arch::asm;
use skyline::hooks::{Region,getRegionAddress};
use std::sync::atomic::{AtomicUsize, Ordering};
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
use skyline::libc::c_void;
use crate::moveset::ui::ui_manager::UiManager;
use crate::moveset::misc::*;
use crate::moveset::kamui::auto_turn;


pub static mut IDX: usize = 0;

// flags

// rs installs
mod global;
mod kamui;
mod misc;
mod hooks;
mod articles;
mod ui;
pub mod consts;
pub use consts::*;

//cancels
pub unsafe extern "C" fn cancels(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);

    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
    let cat3 = ControlModule::get_command_flag_cat(module_accessor, 2);
    let cat4 = ControlModule::get_command_flag_cat(module_accessor, 3);


    if CancelModule::is_enable_cancel(module_accessor) 
    || fighter.global_table[IS_STOP].get_bool() // cant cancel during hitstop
    || !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){ // cancels only if hit
        return;
    }

    //jump cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status){
        if !((motion_kind == hash40("attack_air_b")) || (motion_kind == hash40("attack_air_lw"))){ // cant jump cancel out of bair
            let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            let max_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            let remaining_jumps= max_jump_count - jump_count;
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                if remaining_jumps > 0{
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
                        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);    
                    }else{
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                    }
                    return;
                }
            }
        }
    }
    


    let work_module = *(fighter.module_accessor as *const *const u64).add(0x50 / 8);
    if [
    *FIGHTER_STATUS_KIND_ATTACK
    ,*FIGHTER_STATUS_KIND_ATTACK_100
    ,*FIGHTER_STATUS_KIND_ATTACK_HI3
    ,*FIGHTER_STATUS_KIND_ATTACK_HI4
    ,*FIGHTER_STATUS_KIND_ATTACK_LW3
    ,*FIGHTER_STATUS_KIND_ATTACK_S3
    ,*FIGHTER_STATUS_KIND_ATTACK_S4
    ,*FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status){
        WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
        WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);

        if fighter.sub_transition_group_check_special_command().get_bool(){
            return;
        }


        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            if final_smash_checks(work_module) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FINAL, true);
            } else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
            return;
        }
    }

    if [*FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status){
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
            let control = ControlModule::get_attack_air_kind(module_accessor);
            let aerials = [*FIGHTER_COMMAND_ATTACK_AIR_KIND_B,*FIGHTER_COMMAND_ATTACK_AIR_KIND_F,*FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,*FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,*FIGHTER_COMMAND_ATTACK_AIR_KIND_N];
            for attack_air in aerials{
                if control == attack_air{
                    if !(motion_kind == hash40("attack_air_b")){ //cant cancel bair into any other aerials
                        if !(motion_kind == hash40("attack_air_lw")
                        && control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW) { //cant cancel dair into itself
                            if !(motion_kind == hash40("attack_air_f")
                            && control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F) { // cant cancel fair into itself
                                ControlModule::set_attack_air_kind(module_accessor,attack_air);
                                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status){
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 ) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 ) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK, false);
            return;
        }
    }
    if [
    *FIGHTER_STATUS_KIND_ATTACK_HI3
    ,*FIGHTER_STATUS_KIND_ATTACK_LW3
    ,*FIGHTER_STATUS_KIND_ATTACK_S3
    ].contains(&status){
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 ) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0  {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            return;
        }
    }
    return;
}
// opff
pub unsafe extern "C" fn kamui(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = fighter.module_accessor;
        cancels(fighter);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let mut can_rc = true;
        //if !WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON){
        //    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 20);
        //}
        //hurtboxes
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY || !smash::app::sv_information::is_ready_go() {
            let custom_hurtboxes = [
                //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
                [hash40("hip") as f64, -0.3, 0.0, 0.0, -0.3, 0.0, 0.0, 6.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("waist") as f64, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 7.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_HIGH as f64],
                [hash40("head") as f64, 0.6, 0.0, 0.0, 0.9, 0.0, 0.0, 1.6, *COLLISION_PART_HEAD as f64,*HIT_HEIGHT_HIGH as f64],
                [hash40("shoulderl") as f64, 0.3, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_HIGH as f64],
                [hash40("shoulderr") as f64, 0.3, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_HIGH as f64],
                [hash40("arml") as f64, 0.0, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("armr") as f64, 0.0, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("legl") as f64, 0.2, 0.0, 0.0, 3.8, 0.0, 0.0, 3.4, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("legr") as f64, 0.2, 0.0, 0.0, 3.8, 0.0, 0.0, 3.4, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("kneel") as f64, 0.2, 0.0, 0.0, 4.6, 0.0, 0.0, 2.4, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("kneer") as f64, 0.2, 0.0, 0.0, 4.6, 0.0, 0.0, 2.4, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64]
            ];
            let mut f = 0;
            for i in custom_hurtboxes {
                let mut vec1 = Vector3f{x: i[1] as f32, y: i[2] as f32, z: i[3] as f32};
                let mut vec2 = Vector3f{x: i[4] as f32, y: i[5] as f32, z: i[6] as f32};
                FighterUtil::set_hit_data(module_accessor,f,0,&vec1,&vec2,i[7] as f32,Hash40::new_raw(i[0] as u64),smash::app::CollisionPart(i[8] as i32),smash::app::HitHeight(i[9] as i32),smash::app::HitStatus(*HIT_STATUS_NORMAL),smash::app::CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
                f += 1;
            }
        }
        // set_scale
        ModelModule::set_scale(module_accessor, 1.0);
        AttackModule::set_attack_scale(module_accessor, 1.0, true);
        GrabModule::set_size_mul(module_accessor, 1.0);


        // roman cancel penalty
        if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) == 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.0);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }else if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) > 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.8);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }
        // roman cancels
        let is_stop = fighter.global_table[IS_STOP].get_bool();
        let prev_status_kind = StatusModule::prev_status_kind(module_accessor,0);
        let is_input_pressed = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);

        let roman_cancellable_status =[
            *FIGHTER_STATUS_KIND_APPEAL
            ,*FIGHTER_STATUS_KIND_ATTACK
            ,*FIGHTER_STATUS_KIND_ATTACK_100
            ,*FIGHTER_STATUS_KIND_ATTACK_AIR
            ,*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
            ,*FIGHTER_STATUS_KIND_LANDING_LIGHT
            ,*FIGHTER_STATUS_KIND_ATTACK_DASH
            ,*FIGHTER_STATUS_KIND_ATTACK_HI3
            ,*FIGHTER_STATUS_KIND_ATTACK_HI4
            ,*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD
            ,*FIGHTER_STATUS_KIND_ATTACK_HI4_START
            ,*FIGHTER_STATUS_KIND_ATTACH_WALL
            ,*FIGHTER_STATUS_KIND_ATTACH_WALL_WAIT
            ,*FIGHTER_STATUS_KIND_ATTACK_LW3
            ,*FIGHTER_STATUS_KIND_ATTACK_LW4
            ,*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
            ,*FIGHTER_STATUS_KIND_ATTACK_LW4_START
            ,*FIGHTER_STATUS_KIND_ATTACK_S3
            ,*FIGHTER_STATUS_KIND_ATTACK_S4
            ,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
            ,*FIGHTER_STATUS_KIND_ATTACK_S4_START
            ,*FIGHTER_STATUS_KIND_SQUAT
            ,*FIGHTER_STATUS_KIND_SQUAT_B
            ,*FIGHTER_STATUS_KIND_SQUAT_F
            ,*FIGHTER_STATUS_KIND_SQUAT_RV
            ,*FIGHTER_STATUS_KIND_SQUAT_WAIT
            ,*FIGHTER_STATUS_KIND_SPECIAL_HI
            ,*FIGHTER_STATUS_KIND_SPECIAL_LW
            ,*FIGHTER_STATUS_KIND_SPECIAL_S
            ,*FIGHTER_STATUS_KIND_SPECIAL_N
            ,*FIGHTER_STATUS_KIND_RUN
            ,*FIGHTER_STATUS_KIND_RUN_BRAKE
            ,*FIGHTER_STATUS_KIND_MISS_FOOT
            ,*FIGHTER_STATUS_KIND_OTTOTTO
            ,*FIGHTER_STATUS_KIND_OTTOTTO_WAIT
            ,*FIGHTER_STATUS_KIND_JUMP
            ,*FIGHTER_STATUS_KIND_JUMP_AERIAL
            ,*FIGHTER_STATUS_KIND_JUMP_SQUAT
            ,*FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOP
            ,*FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP
            ,*FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP
            ,*FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL
            ,*FIGHTER_STATUS_KIND_ITEM_SCREW_FALL
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_FLY
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP_SQUAT
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP_AERIAL
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_LANDING
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_TURN
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_B
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_B
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_F
            ,*FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_F
            ,*FIGHTER_STATUS_KIND_ITEM_SWING
            ,*FIGHTER_STATUS_KIND_ITEM_STARRING_SHOOT
            ,*FIGHTER_STATUS_KIND_ITEM_STARRING
            ,*FIGHTER_STATUS_KIND_ITEM_SWING_DASH
            ,*FIGHTER_STATUS_KIND_ITEM_SWING_S3
            ,*FIGHTER_STATUS_KIND_ITEM_SWING_S4
            ,*FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD
            ,*FIGHTER_STATUS_KIND_ITEM_SWING_S4_START
            ,*FIGHTER_STATUS_KIND_GUARD
            ,*FIGHTER_STATUS_KIND_GUARD_DAMAGE
            ,*FIGHTER_STATUS_KIND_GUARD_OFF
            ,*FIGHTER_STATUS_KIND_GUARD_ON
            ,*FIGHTER_STATUS_KIND_ESCAPE
            ,*FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
            ,*FIGHTER_STATUS_KIND_ESCAPE_AIR
            ,*FIGHTER_STATUS_KIND_ESCAPE_F
            ,*FIGHTER_STATUS_KIND_ESCAPE_B
            ,*FIGHTER_STATUS_KIND_FALL
            ,*FIGHTER_STATUS_KIND_FALL_AERIAL
            ,*FIGHTER_STATUS_KIND_DETACH_WALL
            ,*FIGHTER_STATUS_KIND_DETACH_WALL_FALL
            ,*FIGHTER_STATUS_KIND_DETACH_WALL_JUMP
            ,*FIGHTER_STATUS_KIND_DASH
            ,*FIGHTER_STATUS_KIND_CATCH
            ,*FIGHTER_STATUS_KIND_CATCH_JUMP
            ,*FIGHTER_STATUS_KIND_CATCH_ATTACK
            ,*FIGHTER_STATUS_KIND_CATCH_DASH
            ,*FIGHTER_STATUS_KIND_CATCH_PULL
            ,*FIGHTER_STATUS_KIND_CATCH_TURN
            ,*FIGHTER_STATUS_KIND_CATCH_JUMP
            ,*FIGHTER_STATUS_KIND_CATCH_WAIT
            ,*FIGHTER_STATUS_KIND_WALL_JUMP
            ,*FIGHTER_STATUS_KIND_TURN
            ,*FIGHTER_STATUS_KIND_TURN_DASH
            ,*FIGHTER_STATUS_KIND_TURN_RUN
            ,*FIGHTER_STATUS_KIND_TURN_RUN_BRAKE
            //,*FIGHTER_STATUS_KIND_STANDBY            //this is the transitioning period between dead and not
            ,*FIGHTER_STATUS_KIND_SWIM
            ,*FIGHTER_STATUS_KIND_SWIM_DIVE
            ,*FIGHTER_STATUS_KIND_SWIM_DROWN
            ,*FIGHTER_STATUS_KIND_SWIM_END
            ,*FIGHTER_STATUS_KIND_SWIM_JUMP
            ,*FIGHTER_STATUS_KIND_SWIM_RISE
            ,*FIGHTER_STATUS_KIND_SWIM_TURN
            ,*FIGHTER_STATUS_KIND_SWIM_UP
            ,*FIGHTER_STATUS_KIND_SWIM_WAIT
            ,*FIGHTER_STATUS_KIND_THROW
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_JUMP_END
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END
            ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP
            ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START
            ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP
            ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE
            ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE
            ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD
            ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END
            ,FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER
            ,FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF
            ,FIGHTER_KAMUI_STATUS_KIND_AIR_THROW];

        let blue_rc_status_list = 
            [*FIGHTER_STATUS_KIND_WAIT
            ,*FIGHTER_STATUS_KIND_WALK
            ,*FIGHTER_STATUS_KIND_WALK_BRAKE
            ,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN
            ,FIGHTER_KAMUI_STATUS_KIND_WALK_BACK
            ,FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK
            ,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN
            ,*FIGHTER_STATUS_KIND_TURN
            ,*FIGHTER_STATUS_KIND_RUN
            ,*FIGHTER_STATUS_KIND_RUN_BRAKE
            ,*FIGHTER_STATUS_KIND_JUMP
            ,*FIGHTER_STATUS_KIND_JUMP_AERIAL
            ,*FIGHTER_STATUS_KIND_JUMP_SQUAT
            ,*FIGHTER_STATUS_KIND_FALL
            ,*FIGHTER_STATUS_KIND_DAMAGE_FALL
            ,*FIGHTER_STATUS_KIND_GUARD
            ,*FIGHTER_STATUS_KIND_GUARD_ON
            ,*FIGHTER_STATUS_KIND_GUARD_OFF
            ,*FIGHTER_STATUS_KIND_SQUAT
            ,*FIGHTER_STATUS_KIND_SQUAT_B
            ,*FIGHTER_STATUS_KIND_SQUAT_F
            ,*FIGHTER_STATUS_KIND_SQUAT_RV
            ,*FIGHTER_STATUS_KIND_SQUAT_WAIT
            ,*FIGHTER_STATUS_KIND_MISS_FOOT
            ,*FIGHTER_STATUS_KIND_OTTOTTO
            ,*FIGHTER_STATUS_KIND_OTTOTTO_WAIT];



        if !roman_cancellable_status.contains(&status_kind)
        || (prev_status_kind == FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL
            && StatusModule::is_changing(module_accessor)) 
        || !UiManager::is_tension_available(get_ui_index_from_entry_id(entry_id))
        || (CatchModule::is_catch(module_accessor) && !AttackModule::is_hit_abs(module_accessor)){
            can_rc = false;
        }   

        if can_rc{
            if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC)
            && !( is_stop ){
                //red rc
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 25, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 40, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 20,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,4.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 0,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            }

            if MotionModule::motion_kind(module_accessor) == smash::hash40("just_shield_off")
            && is_input_pressed{
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC);
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 12, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 9, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 9,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor, 8.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 14,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            } 

            if (is_stop || AttackModule::is_hit_abs(module_accessor))
            && (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_hit_abs(module_accessor))
            && is_input_pressed{
                //red rc 
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC);
                if get_rc_direction(fighter) != 0{
                    WorkModule::set_int(module_accessor,get_rc_direction(fighter),FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_ROMAN_CANCEL_BUFFER_DIRECTION);
                }
                return;
            } else if (blue_rc_status_list.contains(&status_kind) 
            || ((MotionModule::frame(module_accessor) <= 3.0) && blue_rc_status_list.contains(&prev_status_kind) && (status_kind != FIGHTER_STATUS_KIND_FINAL)) )
            && is_input_pressed
            && !( is_stop ) {
                //blue rc
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC);
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 20, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 45, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 29,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,4.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 6,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            } else if is_input_pressed
            && !( is_stop ) {
                //purple rc
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC);
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 20, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 19, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 9,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,2.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 6,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            }
        }
    }
}




// command input shit
pub unsafe extern "C" fn pot_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK){
        // potemkin buster input
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF.into(), true.into());
            return true.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S{
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF.into(), true.into());
            return true.into();
        }
    }else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_KARA);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
            return true.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_KARA);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
            return true.into();
        }

        let work_module = *(fighter.module_accessor as *const *const u64).add(0x50 / 8);
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
        && final_smash_checks(work_module){
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO);
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            return true.into();
        }
    }
    // heat tackle input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND  != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND){
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_INPUT);
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD.into(), true.into());
        return true.into();
    }
    //forward megafist input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_INPUT);
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F.into(), true.into());
        return true.into();
    }
    // back megafist input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_INPUT);
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B.into(), true.into());
        return true.into();
    }
    return false.into();
}

pub unsafe extern "C" fn blank_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 1.into();
}

// agent init
unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    let fighter_kind = fighter.global_table[KIND].get_i32();
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(auto_turn as *const () as _)); //enables auto turn
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(blank_special_s as *const () as _));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(pot_check_special_command as *const () as _)); //enables special commands

    WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
}


pub unsafe extern "C" fn is_potemkin(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *module_accessor) == *FIGHTER_KIND_KAMUI{
        let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        crate::MARKED_COLORS[color as usize]    
    }else{
        false
    }
}

pub unsafe extern "C" fn is_potemkin_svinfo(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *module_accessor) == *FIGHTER_KIND_KAMUI{
        let entry_id = sv_battle_object::entry_id((*module_accessor).battle_object_id);
        let info = smash::cpp::root::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32));
        let color = smash::app::lua_bind::FighterInformation::fighter_color(info) as i32;    
        crate::MARKED_COLORS[color as usize]
    }else{
        false
    }
}

pub extern "C" fn get_costume_vector() -> Vec<usize> {
    let mut costume = &mut Vec::new();
    unsafe{
        for i in 0..crate::MARKED_COLORS.len() {
            if crate::MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    return costume.to_vec();
}

pub fn install() {

    let mut costume = &mut get_costume_vector();

    unsafe {
        let fdb = smashline::clone_weapon(
            "kamui", // Fighter Name
            "fdb", // Cloned Article Name
            *WEAPON_KIND_KAMUI_RYUSENSYA, // Base Article Kind
            true // Use Original Code
        );

        FIGHTER_KAMUI_GENERATE_ARTICLE_FDB = fdb.generate_id;
        WEAPON_KIND_KAMUI_FDB = fdb.weapon_id;

        smashline::update_weapon_count(
            *FIGHTER_KIND_KAMUI, // Fighter Kind
            FIGHTER_KAMUI_GENERATE_ARTICLE_FDB, // Generate Article ID
            2 // New Max Spawn Count
        );

    }




    let agent_kamui = &mut Agent::new("kamui");
    let agent_kamui = agent_kamui.set_costume(costume.to_vec());

    agent_kamui.on_line(Main, kamui);
    agent_kamui.on_start(agent_init);
    agent_kamui.on_start(on_start);
    kamui::install(agent_kamui);
    agent_kamui.install();


    let agent_waterdragon = &mut Agent::new("kamui_waterdragon");
    let agent_waterdragon = agent_waterdragon.set_costume(costume.to_vec());

    articles::waterdragon::install(agent_waterdragon);
    agent_waterdragon.install();
    
  

    let agent_dragonhand= &mut Agent::new("kamui_dragonhand");
    let agent_dragonhand = agent_dragonhand.set_costume(costume.to_vec());
    articles::dragonhand::install(agent_dragonhand);
    agent_dragonhand.install();



    let agent_ryusensya = &mut Agent::new("kamui_fdb");
    let agent_ryusensya = agent_ryusensya.set_costume(costume.to_vec());
    articles::ryusensya::install(agent_ryusensya);
    agent_ryusensya.install();



    let mut all = &mut Vec::new();
    for k in 8..256{
        all.push(k);
    }
    let agent_global_all = &mut Agent::new("fighter");
    let agent_global_all = agent_global_all.set_costume(all.to_vec());
    global::install(agent_global_all);
    agent_global_all.install();



    let agent_global = &mut Agent::new("fighter");
    global::install(agent_global);
    agent_global.install();


    ui::install();
    hooks::install();
}