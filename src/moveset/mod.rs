use smash::hash40;
use smash_script::macros::*;
use skyline::nn::ro::LookupSymbol;
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
use skyline::libc::c_void;
use crate::moveset::ui::ui_manager::UiManager;
use crate::moveset::misc::*;


pub static mut IDX: usize = 0;

// flags
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY: i32 = 0x200000ec;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_GRABBED_ENEMY: i32 = 0x200000ed;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RELEASE_ENEMY: i32 = 0x200000ee;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH: i32 = 0x200000ee;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY: i32 = 0x200000ef;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM: i32 = 0x200000f0;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL: i32 = 0x200000f1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_RELEASE: i32 = 0x200000f2;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_ATTACK: i32 = 0x200000f1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_START_NO_DEAD: i32 = 0x200000f3;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP: i32 = 0x200000f4;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP: i32 = 0x200000f5;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND : i32 = 0x200000f6;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE : i32 = 0x200000f7;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT : i32 = 0x200000f8;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY : i32 = 0x200000f9;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC : i32 = 0x200000fa;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC : i32 = 0x200000fb;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC : i32 = 0x200000fc;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC : i32 = 0x200000fd;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE : i32 = 0x200000fe;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_DETECTED : i32 = 0x200000ff;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED : i32 = 0x20000100;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SOUND_ID_KILL : i32 = 0x20000104;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME : i32 = 0x20000105;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_RC_DASH : i32 = 0x20000106;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_STOP_FINAL_I_FRAMES : i32 = 0x20000107;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON : i32 = 0x20000108;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED : i32 = 0x20000109;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FLIP : i32 = 0x2000010a;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON : i32 = 0x2000010b;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_CLOCK_EFFECT : i32 = 0x2000010c;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START : i32 = 0x2000010d;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START : i32 = 0x2000010e;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START : i32 = 0x2000010f;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START : i32 = 0x20000110;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SET_ADJUST_POST_CIRCLE : i32 = 0x20000111;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO : i32 = 0x20000112;


pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_INITIAL_SLOW : i32 = 0x20000113;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_END_SLOW : i32 = 0x20000114;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DEATH_IMMINENT : i32 = 0x20000115;



// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK : i32 = 0x20000007;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL : i32 = 0x20000008;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT : i32 = 0x20000009;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_KILL_YOURSELF_NOW : i32 = 0x2000000a;




// floats
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL: i32 = 0x60;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER: i32 = 0x61;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FLIP_SELECTOR: i32 = 0x62;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_X: i32 = 0x63;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_Y: i32 = 0x64;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_LIMITING_SPEED: i32 = 0x65;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_SPEED_MUL: i32 = 0x66;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_X_SPEED: i32 = 0x67;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_Y_SPEED: i32 = 0x68;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT: i32 = 0x69;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_ROMAN_CANCEL_METER_DRAIN: i32 = 0x6a;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_OPPONENT_Z_SCALE: i32 = 0x6b;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE: i32 = 0x6c;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_X: i32 = 0x6d;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_Y: i32 = 0x6e;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED: i32 = 0x6f;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL: i32 = 0x70;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_POTEMKIN_Z_SCALE: i32 = 0x71;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_X_JUMP_SPEED: i32 = 0x72;

// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_BOOM_POWER: i32 = 0x4;

// ints
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID: i32 = 0x100000c1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID: i32 = 0x100000c2;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI: i32 = 0x100000c3;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_ARTICLE_ID: i32 = 0x100000c4;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FINAL_MOTION_SHARE: i32 = 0x100000c5;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER: i32 = 0x100000c6;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION: i32 = 0x100000c7;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME: i32 = 0x100000c8;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME: i32 = 0x100000c9;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER: i32 = 0x100000ca;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME: i32 = 0x100000cb;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID: i32 = 0x100000cc;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID: i32 = 0x100000cd;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER: i32 = 0x10000524;

// new statuses
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START: i32 = 0x1f4;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP: i32 = 0x1f5;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE: i32 = 0x1f6;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START: i32 = 0x1f7;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP: i32 = 0x1f8;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK: i32 = 0x1f9;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP: i32 = 0x1fa;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE: i32 = 0x1fb;
pub const FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL: i32 = 0x1fc;
pub const FIGHTER_KAMUI_STATUS_KIND_WIN: i32 = 0x1fd;
pub const FIGHTER_KAMUI_STATUS_KIND_AIR_THROW: i32 = 0x1fe;



pub const FIGHTER_KAMUI_STATUS_KIND_WALK_BACK: i32 = 0x200;
pub const FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN: i32 = 0x201;
pub const FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK: i32 = 0x202;


pub const FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF: i32 = 0x203;
pub const FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER: i32 = 0x204;

pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD: i32 = 0x205;
pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE: i32 = 0x206;
pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END: i32 = 0x207;

pub const FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN: i32 = 0x208;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM: i32 = 0x1;



// new common statuses
pub const FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT: i32 = 0x555;
pub const FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT: i32 = 0x556;
pub const FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT: i32 = 0x557;
pub const FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT: i32 = 0x558;
pub const FIGHTER_STATUS_KIND_POT_REBOUND_JUMP: i32 = 0x559;


// global table consts
pub const KIND:                     i32 = 0x2;
pub const CMD_CAT4:                 i32 = 0x23;
pub const CHECK_SPECIAL_COMMAND:    i32 = 0x3C;
pub const SITUATION_KIND:           i32 = 0x16;
pub const PREV_SITUATION_KIND:      i32 = 0x17;
pub const STICK_X:                  i32 = 0x1A;
pub const STICK_Y:                  i32 = 0x1B;
pub const PREV_STATUS_KIND:         i32 = 0xA;
pub const STATUS_KIND:              i32 = 0xB;
pub const IS_STOP:                  i32 = 0x8;










// rs installs
mod global;
mod kamui;
mod misc;
mod hooks;
mod articles;
mod ui;


//this does... something......... probably
extern "C" {
    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
}


pub unsafe extern "C" fn camera_checks(fighter: &mut L2CAgentBase) -> bool {
    let ret = !is_flat_stage() && FighterCutInManager__is_one_on_one();
    return ret;
}

pub unsafe extern "C" fn attack_sound_randomizer(fighter: &mut L2CAgentBase, mut attack_type: &str) -> u64 {
    let mut random = smash::app::sv_math::rand(hash40("fighter"), 6);
    let mut sound_name = ["vc_pot_attack_".to_string(),attack_type.to_string()].join("");
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID) == random {
        random = (random + 1)%6;
    }
    match random{
        0 => sound_name = [sound_name,"_a".to_string()].join(""),
        1 => sound_name = [sound_name,"_b".to_string()].join(""),
        2 => sound_name = [sound_name,"_c".to_string()].join(""),
        3 => sound_name = [sound_name,"_d".to_string()].join(""),
        4 => sound_name = [sound_name,"_e".to_string()].join(""),
        5 => sound_name = [sound_name,"_f".to_string()].join(""),
        _ => (),
    }
    WorkModule::set_int(fighter.module_accessor, random, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID);
    return SoundModule::play_se(fighter.module_accessor, Hash40::new(sound_name.as_str()), true, false, false, false, enSEType(0));
}

// used for sound randomization
pub unsafe extern "C" fn sound_selector(fighter: &mut L2CAgentBase, mut sound_name: String, num_of_vars: i32, is_sequence: bool, continue_sequence: bool, sequence_order_number: i32) -> u64 {
    let mut random = smash::app::sv_math::rand(hash40("fighter"), num_of_vars);
    let random2 = smash::app::sv_math::rand(hash40("fighter"), num_of_vars-1);
    let last_sound_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID);
    
    if !continue_sequence{
        if last_sound_id == random {
            random = (random + 1)%(num_of_vars);
        }
        if is_sequence{
            for _ in 0..3 {
                sound_name.pop();
            }
            match random{
                0 => sound_name = [sound_name,"a_1".to_string()].join(""),
                1 => sound_name = [sound_name,"b_1".to_string()].join(""),
                2 => sound_name = [sound_name,"c_1".to_string()].join(""),
                3 => sound_name = [sound_name,"d_1".to_string()].join(""),
                4 => sound_name = [sound_name,"e_1".to_string()].join(""),
                5 => sound_name = [sound_name,"f_1".to_string()].join(""),
                _ => (),
            }
        }else{
            sound_name.pop();
            match random{
                0 => sound_name = [sound_name,"a".to_string()].join(""),
                1 => sound_name = [sound_name,"b".to_string()].join(""),
                2 => sound_name = [sound_name,"c".to_string()].join(""),
                3 => sound_name = [sound_name,"d".to_string()].join(""),
                4 => sound_name = [sound_name,"e".to_string()].join(""),
                5 => sound_name = [sound_name,"f".to_string()].join(""),
                _ => (),
            }
        }
        WorkModule::set_int(fighter.module_accessor, random, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID);
    } else {
        for _ in 0..3 {
            sound_name.pop();
        }
        match last_sound_id{
            0 => sound_name = [sound_name,"a_".to_string()].join(""),
            1 => sound_name = [sound_name,"b_".to_string()].join(""),
            2 => sound_name = [sound_name,"c_".to_string()].join(""),
            3 => sound_name = [sound_name,"d_".to_string()].join(""),
            4 => sound_name = [sound_name,"e_".to_string()].join(""),
            5 => sound_name = [sound_name,"f_".to_string()].join(""),
            _ => (),
        }
        sound_name = [sound_name,sequence_order_number.to_string()].join("");
    }
    return hash40(sound_name.as_str())
}


pub unsafe extern "C" fn rc_speed_float_setter(fighter: &mut L2CFighterCommon) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    WorkModule::set_float(fighter.module_accessor, speed_x, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_X_SPEED);
    WorkModule::set_float(fighter.module_accessor, speed_y, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_Y_SPEED);
}

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
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND) != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE){
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO);

            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);
            return;
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND) != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF, false);
            return;
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) != 0{
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, false);
            return;
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) != 0{
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, false);
            return;
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) != 0{
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, false);
            return;
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
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
        if![
        *FIGHTER_STATUS_KIND_ATTACK_HI4
        ,*FIGHTER_STATUS_KIND_ATTACK_S4
        ].contains(&status){
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
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status){
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
            let control = ControlModule::get_attack_air_kind(module_accessor);
            let aerials = [*FIGHTER_COMMAND_ATTACK_AIR_KIND_B,*FIGHTER_COMMAND_ATTACK_AIR_KIND_F,*FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,*FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,*FIGHTER_COMMAND_ATTACK_AIR_KIND_N];
            for attack_air in aerials{
                if control == attack_air{
                    if !(motion_kind == hash40("attack_air_lw")
                    && control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW){
                        if !(motion_kind == hash40("attack_air_f")
                        && control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F){
                            ControlModule::set_attack_air_kind(module_accessor,attack_air);
                            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                            return;
                        }
                    }
                }
            }
        }
    }
    if [*FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status){
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
        let prev_status_kind = StatusModule::prev_status_kind(module_accessor,0);
        let mut can_rc = true;
        //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        //&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        //&& [*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN,*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind){
        //    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(), true.into());
        //}

        //hurtboxes
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY || !smash::app::sv_information::is_ready_go() {
            let custom_hurtboxes = [
                //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
                [hash40("hip") as f64, -0.3, 0.0, 0.0, -0.3, 0.0, 0.0, 6.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("waist") as f64, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 7.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("head") as f64, 0.6, 0.0, 0.0, 0.9, 0.0, 0.0, 1.6, *COLLISION_PART_HEAD as f64,*HIT_HEIGHT_HIGH as f64],
                [hash40("shoulderl") as f64, 0.3, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("shoulderr") as f64, 0.3, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("arml") as f64, 0.0, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_ETC as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("armr") as f64, 0.0, 0.0, 0.0, 7.75, 0.0, 0.0, 4.0, *COLLISION_PART_ETC as f64,*HIT_HEIGHT_CENTER as f64],
                [hash40("legl") as f64, 0.2, 0.0, 0.0, 3.8, 0.0, 0.0, 3.4, *COLLISION_PART_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("legr") as f64, 0.2, 0.0, 0.0, 3.8, 0.0, 0.0, 3.4, *COLLISION_PART_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("kneel") as f64, 0.2, 0.0, 0.0, 4.6, 0.0, 0.0, 2.4, *COLLISION_PART_LEGS as f64,*HIT_HEIGHT_LOW as f64],
                [hash40("kneer") as f64, 0.2, 0.0, 0.0, 4.6, 0.0, 0.0, 2.4, *COLLISION_PART_LEGS as f64,*HIT_HEIGHT_LOW as f64]
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
        if is_potemkin(module_accessor) {
            ModelModule::set_scale(module_accessor, 1.0);
            AttackModule::set_attack_scale(module_accessor, 1.0, true);
            GrabModule::set_size_mul(module_accessor, 1.0);
        }


        // roman cancel penalty
        if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) == 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.0);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }else if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) > 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.9);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }

        //roman cancels
        if !(
        [*FIGHTER_STATUS_KIND_APPEAL
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
        ,FIGHTER_KAMUI_STATUS_KIND_AIR_THROW
        ].contains(&status_kind) )
        || (StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL
            && StatusModule::is_changing(module_accessor)) 
        || !UiManager::is_tension_available(get_ui_index_from_entry_id(entry_id)){
            can_rc = false;
        }



        

        if can_rc{
            if MotionModule::motion_kind(module_accessor) == smash::hash40("just_shield_off")
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
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

            let blue_rc_status_list = 
            [*FIGHTER_STATUS_KIND_WAIT
            ,*FIGHTER_STATUS_KIND_WALK
            ,*FIGHTER_STATUS_KIND_WALK_BRAKE
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


            if fighter.global_table[IS_STOP].get_bool()
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
                //red rc 
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC);
                return;
            } else if 
            (blue_rc_status_list.contains(&status_kind) 
            || ((MotionModule::frame(module_accessor) <= 2.0) && blue_rc_status_list.contains(&prev_status_kind) && (status_kind != FIGHTER_STATUS_KIND_FINAL)) )
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && !( fighter.global_table[IS_STOP].get_bool() ) {
                //blue rc
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC);
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 20, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 59, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 29,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,4.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 6,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            } else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && !( fighter.global_table[IS_STOP].get_bool() ) {
                //purple rc
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC);
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 20, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 19, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 9,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,4.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 6,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            }

            if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC)
            && !( fighter.global_table[IS_STOP].get_bool() ){
                //red rc
                rc_speed_float_setter(fighter);
                WorkModule::set_int(module_accessor, 25, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);
                WorkModule::set_int(module_accessor, 39, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
                WorkModule::set_int(module_accessor, 19,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME);
                WorkModule::set_float(module_accessor,4.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL);
                WorkModule::set_int(module_accessor, 1,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME);
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL, true);
                return;
            }
        }
    }
}




// command input shit
pub unsafe extern "C" fn pot_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    // potemkin buster input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE))
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND){
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

    //// heat tackle input
    //if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1 != 0
    //&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND){
    //    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD.into(), true.into());
    //    return true.into();
    //}
    // heat knuckle input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND  != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND){
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD.into(), true.into());
        return true.into();
    }
    //forward megafist input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F.into(), true.into());
        return true.into();
    }
    // back megafist input
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B.into(), true.into());
        return true.into();
    }
    //hammerfall input
    //if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
    //&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    //&& fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND{
    //    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
    //    return true.into();
    //}
    return false.into();
}


// agent init
unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    let fighter_kind = fighter.global_table[KIND].get_i32();
    fighter.global_table[0x3E].assign(&L2CValue::Ptr(auto_turn as *const () as _)); //enables auto turn
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






pub fn install() {



    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..crate::MARKED_COLORS.len() {
            if crate::MARKED_COLORS[i] {
                costume.push(i);
            }
        }
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



    let agent_ryusensya = &mut Agent::new("kamui_ryusensya");
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