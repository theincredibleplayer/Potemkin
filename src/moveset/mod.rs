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
use skyline::libc::c_void;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;

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


//vtable patching stuff
pub static mut KAMUI_VTABLE_ENTRIES: [usize; 146] = [0; 146];
pub static mut KAMUI_WEAPON_VTABLE_ENTRIES: [usize; 102] = [0; 102];


fn patch_vtable_function(offset: usize, function: u64) {
    // println!("function ptr: {:#x}", function as u64);
    let low = (function as u64 & 0xFFFFFFFF) as u32;
    // println!("Low to Big Endian: {:#x} > {:#?}", low, low.to_be_bytes());
    let high = (function as u64 >> 32) as u32;
    // println!("High to Big Endian: {:#x} > {:#?}", high, high.to_be_bytes());
    let _ = skyline::patching::Patch::in_text(offset).data(low);
    let _ = skyline::patching::Patch::in_text(offset + 0x4).data(high);
}







// rs installs
mod hooks;
use hooks::*;
mod hooks_vtable;
use hooks_vtable::*;
mod throw;
use throw::*;
mod taunt;
use taunt::*;
mod speciallw;
use speciallw::*;
mod speciallw_acmd;
use speciallw_acmd::*;
mod specialhi;
use specialhi::*;
mod specialhi_acmd;
use specialhi_acmd::*;
mod specialairhi;
use specialairhi::*;
mod specialairhi_acmd;
use specialairhi_acmd::*;
mod normals;
use normals::*;
mod tilts;
use tilts::*;
mod smash_attacks;
use smash_attacks::*;
mod aerials;
use aerials::*;
mod air_throw;
use air_throw::*;
mod specials;
use specials::*;
mod specials_acmd;
use specials_acmd::*;
mod specialn;
use specialn::*;
mod specialn_acmd;
use specialn_acmd::*;
mod finalsmash;
use finalsmash::*;
mod finalsmash_acmd;
use finalsmash_acmd::*;
mod event_structs;
use event_structs::*;
mod caught;
use caught::*;
mod roman_cancel;
use roman_cancel::*;
mod roman_cancel_acmd;
use roman_cancel_acmd::*;
mod lowercase_letters;
use lowercase_letters::*;
mod misc;
use misc::*;
mod extra_functions;
use extra_functions::*;
mod win;
use win::*;
mod ui_manager;
use ui_manager::*;
mod dual_meter;
use dual_meter::*;
mod autoturn;
use autoturn::*;
mod potbuster;
use potbuster::*;
mod command_input;
use command_input::*;
mod command_structs;
use command_structs::*;
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
    || !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){ // cancels only if hit or collided with a shield
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
        && smash::app::lua_bind::FighterManager::is_final(singletons::FighterManager()){
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
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND) != 0{
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF, false);
            return;
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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
                        ControlModule::set_attack_air_kind(module_accessor,attack_air);
                        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                        return;
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
            if (status == *FIGHTER_STATUS_KIND_ATTACK_100) && AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_SHIELD){ // disable jab from cancelling into itself when colliding with a shield
                return;
            }
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

        if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) == 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.0);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }else if WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER) > 0{
            UiManager::set_penalty(get_ui_index_from_entry_id(entry_id), 0.9);
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER);
        }
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
            ,*FIGHTER_STATUS_KIND_SQUAT_WAIT];


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
    && smash::app::lua_bind::FighterManager::is_final(singletons::FighterManager())
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND){
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
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
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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
    fighter.global_table[0x3E].assign(&L2CValue::Ptr(auto_turn as *const () as _));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(pot_check_special_command as *const () as _));
    WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
}


unsafe extern "C" fn opff_global(fighter: &mut L2CFighterCommon) {
    let fighter_kind = fighter.global_table[KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if !is_potemkin(fighter.module_accessor) {
        UiManager::disable_ui(get_ui_index_from_entry_id(entry_id));
    }
}

unsafe extern "C" fn agent_init_global(fighter: &mut L2CFighterCommon) {
    if smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager() as _) > 4 {
        UiManager::player_count_over_4(true);
    }else{
        UiManager::player_count_over_4(false);
    }
}


pub unsafe extern "C" fn is_potemkin(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    crate::MARKED_COLORS[color as usize]
}







pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
        LookupSymbol(
            &mut ITEM_MANAGER,
            "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..crate::MARKED_COLORS.len() {
            if crate::MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("kamui").set_costume(costume.to_vec())
    //agent frame/opff
    .on_line(Main, kamui)
    .on_start(agent_init)
    .on_start(on_start)

    //autoturn
    .status(Main      , *FIGHTER_STATUS_KIND_WAIT, wait_main)
    .status(Pre       ,  *FIGHTER_STATUS_KIND_WAIT, wait_pre)

    .status(Main      , *FIGHTER_STATUS_KIND_WALK, walk_main)

    //.status(Main      , *FIGHTER_STATUS_KIND_FALL, fall_main)

    .status(Pre       , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_pre)
    .status(Main      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_main)
    .status(Exit      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_exit)
    .status(Init      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_init)
    .status(Exec      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_exec)
    .status(End       , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_end)
    .status(CalcParam , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK, walkback_calc_param)

    .status(Pre       , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK, walk_back_brake_pre)
    .status(Main      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK, walk_back_brake_main)
    .status(Init      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK, walk_back_brake_init)
    .status(Exec      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK, walk_back_brake_exec)
    .status(End       , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK, walk_back_brake_end)

    .status(Pre       ,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN, autoturn_pre)
    .status(Main      ,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN, autoturn_main)
    .status(End       ,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN, autoturn_end)


    .status(Pre       ,FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN, air_autoturn_pre)
    .status(Main      ,FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN, air_autoturn_main)
    .status(End       ,FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN, air_autoturn_end)

    //throw
    .game_acmd("game_throwlw", throw_lw, Priority::Low)
    .effect_acmd("effect_throwlw", effect_throw_lw, Priority::Low)
    .sound_acmd("sound_throwlw", sound_throw_lw, Priority::Low)
    .expression_acmd("expression_throwlw", expression_throwlw, Priority::Low)

    .game_acmd("game_throwf", game_throwf, Priority::Low)
    .effect_acmd("effect_throwf", effect_throwf, Priority::Low)
    .sound_acmd("sound_throwf", sound_throwf, Priority::Low)
    .expression_acmd("expression_throwf", expression_throwf, Priority::Low)

    .game_acmd("game_throwb", game_throwb, Priority::Low)
    .effect_acmd("effect_throwb", effect_throwf, Priority::Low)
    .sound_acmd("sound_throwb", sound_throwf, Priority::Low)
    .expression_acmd("expression_throwb", expression_throwf, Priority::Low)

    .game_acmd("game_throwhi", game_throwhi, Priority::Low)
    .effect_acmd("effect_throwhi", effect_throwhi, Priority::Low)
    .sound_acmd("sound_throwhi", sound_throwhi, Priority::Low)
    //.expression_acmd("expression_throwhi", expression_throwlw, Priority::Low)

    //taunt
    .game_acmd("game_appealsr", appeal_s, Priority::Low)
    .effect_acmd("effect_appealsr", effect_appeal, Priority::Low)
    .sound_acmd("sound_appealsr", sound_appeal_s, Priority::Low)

    .game_acmd("game_appealsl", appeal_s, Priority::Low)
    .effect_acmd("effect_appealsl", effect_appeal, Priority::Low)
    .sound_acmd("sound_appealsl", sound_appeal_s, Priority::Low)

    
    .effect_acmd("effect_appealhir", effect_appeal, Priority::Low)
    .sound_acmd("sound_appealhir", sound_appeal_hi, Priority::Low)

    .effect_acmd("effect_appealhil", effect_appeal, Priority::Low)
    .sound_acmd("sound_appealhil", sound_appeal_hi, Priority::Low)

    
    .effect_acmd("effect_appeallwr", effect_appeal, Priority::Low)
    .sound_acmd("sound_appeallwr", sound_appeal_lw, Priority::Low)

    .effect_acmd("effect_appeallwl", effect_appeal, Priority::Low)
    .sound_acmd("sound_appeallwl", sound_appeal_lw, Priority::Low)


    .game_acmd("game_entryl", game_entryl, Priority::Low)
    .effect_acmd("effect_entryl", effect_entryl, Priority::Low)
    .sound_acmd("sound_entryl", sound_entryl, Priority::Low)

    .game_acmd("game_entryr", game_entryl, Priority::Low)
    .effect_acmd("effect_entryr", effect_entryl, Priority::Low)
    .sound_acmd("sound_entryr", sound_entryl, Priority::Low)



    .status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end)

    //special_s
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END, side_b_end_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END, side_b_end)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, side_b_attack_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, side_b_attack)
    .status(Init, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_init)
    .status(Exit, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_exit)
    .status(Exec, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_exec)
    .status(End, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_end)
    .sound_acmd("sound_specials", sound_specialsstart, Priority::Low)
    .game_acmd("game_specialsslide", game_specialsloop, Priority::Low)
    .effect_acmd("effect_specialsslide", effect_specialsloop, Priority::Low)
    .effect_acmd("effect_specials", effect_specials, Priority::Low)
    .game_acmd("game_specialsattack", game_specialsattack, Priority::Low)
    .sound_acmd("sound_specialsattack", sound_specialsattack, Priority::Low)
    .effect_acmd("effect_specialsattack", effect_specialsattack, Priority::Low)

    .sound_acmd("sound_specialsend", sound_specialsend, Priority::Low)

    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, megafist_f_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, megafist_f_main)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING, megafist_f_land_main)
    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, megafist_f_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, megafist_f_main)
    .game_acmd("game_specialsmff", game_specialsmff, Priority::Low)
    .sound_acmd("sound_specialsmff", sound_specialsmff, Priority::Low)

    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING, megafist_f_land_main)


    //special_lw
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init)
    .game_acmd("game_speciallw", special_lw, Priority::Low)
    .effect_acmd("effect_speciallw", effect_special_lw, Priority::Low)
    .sound_acmd("sound_speciallw", sound_special_lw, Priority::Low)

    .game_acmd("game_specialairlw", special_lw, Priority::Low)
    .effect_acmd("effect_specialairlw", effect_special_lw, Priority::Low)
    .sound_acmd("sound_specialairlw", sound_special_lw, Priority::Low)

    //special_hi
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end)

    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, special_hi_start_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, special_hi_end)

    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP, special_hi_loop_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP, special_hi_end)


    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE, special_hi_release_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE, special_hi_end)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE, special_hi_release_exec)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE, special_air_hi_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE, special_air_hi_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE, special_air_hi_end)
    //.status(Exec, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, special_air_hi_hold_exec)
    //.status(Exit, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, special_air_hi_hold_exit)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, special_air_hi_hold_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, special_air_hi_hold_main)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END, special_air_hi_end_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END, special_air_hi_end_main)


    .game_acmd          ("game_specialairhi",              special_air_hi,                                 Priority::Low)
    .sound_acmd         ("sound_specialairhi",             sound_special_air_hi,                           Priority::Low)
    .effect_acmd        ("effect_specialairhi",            effect_special_air_hi,                          Priority::Low)
    .expression_acmd    ("expression_specialairhi",        expression_special_air_hi,                      Priority::Low)

    .game_acmd          ("game_specialhi",              special_hi,                                 Priority::Low)
    .sound_acmd         ("sound_specialhi",             sound_special_hi,                           Priority::Low)
    .effect_acmd        ("effect_specialhi",            effect_special_hi,                          Priority::Low)


    .game_acmd          ("game_specialhistart",         special_hi_start,                           Priority::Low)
    .sound_acmd         ("sound_specialhistart",        sound_special_hi_start,                     Priority::Low)
    .expression_acmd    ("expression_specialhistart",        expression_special_hi_start,                      Priority::Low)

    .game_acmd          ("game_specialhiloop",          special_hi_loop,                            Priority::Low)
    .effect_acmd          ("effect_specialhiloop",          effect_special_hi_loop,                            Priority::Low)
    .expression_acmd          ("expression_specialhiloop",          expression_special_hi_loop,                            Priority::Low)

    .game_acmd          ("game_specialhirelease",       special_hi_release,                         Priority::Low)
    .effect_acmd         ("effect_specialhirelease",      effect_special_hi_release,                   Priority::Low)
    .sound_acmd         ("sound_specialhirelease",      sound_special_hi_release,                   Priority::Low)
    .expression_acmd    ("expression_specialhirelease",        expression_special_hi_release,                      Priority::Low)




    //normals
    .sound_acmd         ("sound_wait2",             sound_wait2,                            Priority::Low)


    .sound_acmd         ("sound_walkslow",             sound_walkslow,                            Priority::Low)
    .effect_acmd        ("effect_walkslow",            effect_walkslow,                          Priority::Low)
    .expression_acmd    ("expression_walkslow",        expression_walkslow,                       Priority::Low)

    .sound_acmd         ("sound_walkmiddle",             sound_walkslow,                            Priority::Low)
    .effect_acmd        ("effect_walkmiddle",            effect_walkslow,                          Priority::Low)
    .expression_acmd    ("expression_walkmiddle",        expression_walkslow,                       Priority::Low)
    
    .sound_acmd         ("sound_walkfast",             sound_walkslow,                            Priority::Low)
    .effect_acmd        ("effect_walkfast",            effect_walkslow,                          Priority::Low)
    .expression_acmd    ("expression_walkfast",        expression_walkslow,                       Priority::Low)


    .sound_acmd         ("sound_walkslowb",             sound_walkslowb,                            Priority::Low)
    .effect_acmd        ("effect_walkslowb",            effect_walkslowb,                          Priority::Low)
    .expression_acmd    ("expression_walkslowb",        expression_walkslowb,                       Priority::Low)

    .sound_acmd         ("sound_walkmiddleb",             sound_walkslowb,                            Priority::Low)
    .effect_acmd        ("effect_walkmiddleb",            effect_walkslowb,                          Priority::Low)
    .expression_acmd    ("expression_walkmiddleb",        expression_walkslowb,                       Priority::Low)
    
    .sound_acmd         ("sound_walkfastb",             sound_walkslowb,                            Priority::Low)
    .effect_acmd        ("effect_walkfastb",            effect_walkslowb,                          Priority::Low)
    .expression_acmd    ("expression_walkfastb",        expression_walkslowb,                       Priority::Low)


    .game_acmd          ("game_slipattack",               game_slipattack,                                  Priority::Low)
    .sound_acmd         ("sound_slipattack",              sound_slipattack,                             Priority::Low)
    .effect_acmd        ("effect_slipattack",             effect_slipattack,                           Priority::Low)
    .expression_acmd    ("expression_slipattack",         expression_slipattack,                        Priority::Low)

    .game_acmd          ("game_downattacku",               game_slipattack,                                  Priority::Low)
    .sound_acmd         ("sound_downattacku",              sound_slipattack,                             Priority::Low)
    .effect_acmd        ("effect_downattacku",             effect_slipattack,                           Priority::Low)
    .expression_acmd    ("expression_downattacku",         expression_slipattack,                        Priority::Low)

    .game_acmd          ("game_downattackd",               game_slipattack,                                  Priority::Low)
    .sound_acmd         ("sound_downattackd",              sound_slipattack,                             Priority::Low)
    .effect_acmd        ("effect_downattackd",             effect_slipattack,                           Priority::Low)
    .expression_acmd    ("expression_downattackd",         expression_slipattack,                        Priority::Low)


    .game_acmd          ("game_attacks3",               game_attacks3,                                  Priority::Low)
    .effect_acmd        ("effect_attacks3",             effect_attacks3,                           Priority::Low)
    .sound_acmd         ("sound_attacks3",             sound_attacks3,                         Priority::Low)


    .game_acmd          ("game_attackhi3",               game_attackhi3,                                Priority::Low)
    .effect_acmd        ("effect_attackhi3",             effect_attackhi3,                         Priority::Low)
    .sound_acmd        ("sound_attackhi3",             sound_attackhi3,                         Priority::Low)


    .game_acmd          ("game_attacklw3",              game_attacklw3,                                 Priority::Low)
    .effect_acmd        ("effect_attacklw3",            effect_attacklw3,                          Priority::Low)
    .sound_acmd        ("sound_attacklw3",             sound_attacklw3,                         Priority::Low)

    //.game_acmd          ("game_attackdash",             attack_dash,                                Priority::Low)
    //.effect_acmd        ("effect_attackdash",           effect_attack_dash,                         Priority::Low)

    .game_acmd          ("game_attack11",               game_attack11,                              Priority::Low)
    .effect_acmd        ("effect_attack11",             effect_attack11,                            Priority::Low)
    .sound_acmd         ("sound_attack11",              sound_attack11,                             Priority::Low)


    .game_acmd          ("game_catch",                  catch,                                      Priority::Low)

    .game_acmd          ("game_catchdash",              catchdash,                                  Priority::Low)

    .game_acmd          ("game_catchturn",              catchturn,                                  Priority::Low)

    .game_acmd          ("game_catchattack",            game_catchattack,                           Priority::Low)

    .game_acmd          ("game_cliffattack",              game_cliffattack,                                 Priority::Low)
    .effect_acmd        ("effect_cliffattack",            effect_cliffattack,                          Priority::Low)

    .game_acmd          ("game_attackhi4",              game_attackhi4,                                 Priority::Low)
    .sound_acmd         ("sound_attackhi4",             sound_attackhi4,                            Priority::Low)
    .effect_acmd        ("effect_attackhi4",            effect_attackhi4,                          Priority::Low)
    .expression_acmd    ("expression_attackhi4",        expression_attackhi4,                       Priority::Low)

    .sound_acmd         ("sound_attackhi4charge",             sound_attackhi4charge,                            Priority::Low)
    .effect_acmd        ("effect_attackhi4charge",            effect_attackhi4charge,                          Priority::Low)
    .expression_acmd    ("expression_attackhi4charge",        expression_attackhi4charge,                       Priority::Low)

    .game_acmd          ("game_attacks4",               game_attacks4,                                  Priority::Low)
    .sound_acmd         ("sound_attacks4",              sound_attacks4,                             Priority::Low)
    .effect_acmd        ("effect_attacks4",             effect_attacks4,                           Priority::Low)
    .expression_acmd    ("expression_attacks4",         expression_attacks4,                        Priority::Low)

    .game_acmd          ("game_attacks4charge",               game_attacks4charge,                                  Priority::Low)
    .sound_acmd         ("sound_attacks4charge",              sound_attacks4charge,                             Priority::Low)
    .effect_acmd        ("effect_attacks4charge",             effect_attacks4charge,                           Priority::Low)
    .expression_acmd    ("expression_attacks4charge",         expression_attacks4charge,                        Priority::Low)


    .game_acmd          ("game_attacklw4",              game_attacklw4,                             Priority::Low)
    .sound_acmd         ("sound_attacklw4",             sound_attacklw4,                            Priority::Low)
    .effect_acmd        ("effect_attacklw4",            effect_attacklw4,                           Priority::Low)

    .sound_acmd         ("sound_attacklw4charge",              sound_attacklw4charge,                             Priority::Low)
    .effect_acmd        ("effect_attacklw4charge",             effect_attacklw4charge,                           Priority::Low)
    .expression_acmd    ("expression_attacklw4charge",         expression_attacklw4charge,                        Priority::Low)


    .game_acmd          ("game_win1a",                  game_win1a,                                 Priority::Low)
    .sound_acmd         ("sound_win1a",                 sound_win1a,                                Priority::Low)

    .game_acmd          ("game_attackairhi",            game_attackairhi,                           Priority::Low)
    .sound_acmd          ("sound_attackairhi",            sound_attackairhi,                           Priority::Low)
    .effect_acmd          ("effect_attackairhi",            effect_attackairhi,                           Priority::Low)

    .game_acmd          ("game_attackairlw",            game_attackairlw,                           Priority::Low)
    .sound_acmd         ("sound_attackairlw",             sound_attackairlw,                          Priority::Low)
    .effect_acmd        ("effect_attackairlw",            effect_attackairlw,                         Priority::Low)

    .game_acmd          ("game_attackairn",            game_attackairn,                           Priority::Low)
    .sound_acmd         ("sound_attackairn",             sound_attackairn,                          Priority::Low)
    .effect_acmd        ("effect_attackairn",            effect_attackairn,                         Priority::Low)


    .game_acmd          ("game_attackairb",             game_attackairb,                            Priority::Low)
    .sound_acmd         ("sound_attackairb",              sound_attackairb,                             Priority::Low)
    .effect_acmd        ("effect_attackairb",             effect_attackairb,                           Priority::Low)
    .expression_acmd    ("expression_attackairb",         expression_attackairb,                        Priority::Low)

    .game_acmd          ("game_attackairf",             game_attackairf,                            Priority::Low)
    .sound_acmd         ("sound_attackairf",              sound_attackairf,                             Priority::Low)
    .effect_acmd        ("effect_attackairf",             effect_attackairf,                           Priority::Low)
    .expression_acmd    ("expression_attackairf",         expression_attackairf,                        Priority::Low)


    .game_acmd          ("game_throwair",             game_throwair,                            Priority::Low)
    .sound_acmd         ("sound_throwair",              sound_throwair,                             Priority::Low)
    //.effect_acmd        ("effect_throwair",             effect_throwair,                           Priority::Low)
    //.expression_acmd    ("expression_throwair",         expression_throwair,                        Priority::Low)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_end)

    
    //roman cancel
    .status(Pre  , FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL,                  roman_cancel_pre)
    .status(Main , FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL,                  roman_cancel_main)
    .status(End  , FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL,                  roman_cancel_end)

    .game_acmd          ("game_romancancel",            game_romancancel,                           Priority::Low)
    .game_acmd          ("game_romancancelair",         game_romancancel,                           Priority::Low)

    //pot buster

    .status(Main, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF, potbusterwhiff_main)

    .status(Main, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, potbuster_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, potbuster_end)

    .game_acmd          ("game_potemkinbuster",             potbuster,                            Priority::Low)
    .sound_acmd         ("sound_potemkinbuster",              sound_potbuster,                             Priority::Low)
    .effect_acmd        ("effect_potemkinbuster",             effect_potbuster,                           Priority::Low)
    .expression_acmd    ("expression_potemkinbuster",         expression_potbuster,                        Priority::Low)

    .game_acmd          ("game_potemkinbusterwhiff",             potbusterwhiff,                            Priority::Low)
    .sound_acmd         ("sound_potemkinbusterwhiff",              sound_potbusterwhiff,                             Priority::Low)

    
    //special_n
    .status(Main , *FIGHTER_STATUS_KIND_SPECIAL_N,                          special_n_main)
    .status(End  , *FIGHTER_STATUS_KIND_SPECIAL_N,                          special_n_end)

    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,               special_n_hold_main)
    .status(End  , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,               special_n_hold_end)

    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,              special_n_shoot_main)
    .status(End  , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,              special_n_shoot_end)


    //.game_acmd("game_finalairlend", game_finalend, Priority::Low)
    .sound_acmd         ("sound_specialnstart",                 sound_specialnstart,                Priority::Low)
    .effect_acmd        ("effect_specialnstart",                effect_specialnstart,               Priority::Low)
    .expression_acmd    ("expression_specialnstart",            expression_specialnstart,           Priority::Low)
                
    .game_acmd          ("game_specialnhold",                   game_specialnhold,                  Priority::Low)
    .sound_acmd         ("sound_specialnhold",                  sound_specialnhold,                 Priority::Low)
    .effect_acmd        ("effect_specialnhold",                 effect_specialnhold,                Priority::Low)
    .expression_acmd    ("expression_specialnhold",             expression_specialnhold,            Priority::Low)
                
    .game_acmd          ("game_specialnfire",                   game_specialnfire,                  Priority::Low)
    .sound_acmd         ("sound_specialnfire",                  sound_specialnfire,                 Priority::Low)
    .effect_acmd        ("effect_specialnfire",                 effect_specialnfire,                Priority::Low)
    .expression_acmd    ("sound_specialnfire",                  sound_specialnfire,                 Priority::Low)

    .sound_acmd         ("sound_specialairnstart",                 sound_specialnstart,                Priority::Low)
    .effect_acmd        ("effect_specialairnstart",                effect_specialnstart,               Priority::Low)
    .expression_acmd    ("expression_specialairnstart",            expression_specialnstart,           Priority::Low)
                
    .game_acmd          ("game_specialairnhold",                   game_specialnhold,                  Priority::Low)
    .sound_acmd         ("sound_specialairnhold",                  sound_specialnhold,                 Priority::Low)
    .effect_acmd        ("effect_specialairnhold",                 effect_specialnhold,                Priority::Low)
    .expression_acmd    ("expression_specialairnhold",             expression_specialnhold,            Priority::Low)
                
    .game_acmd          ("game_specialairnfire",                   game_specialnfire,                  Priority::Low)
    .sound_acmd         ("sound_specialairnfire",                  sound_specialnfire,                 Priority::Low)
    .effect_acmd        ("effect_specialairnfire",                 effect_specialnfire,                Priority::Low)
    .expression_acmd    ("sound_specialairnfire",                  sound_specialnfire,                 Priority::Low)


    //final
    .status(Main , *FIGHTER_STATUS_KIND_FINAL,                              Final_main)
    .status(Exit , *FIGHTER_STATUS_KIND_FINAL,                              Final_exit)


    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY,            finalsceneenter_main)

    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,           finalsceneattack_main)

    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_FINAL_END,                    finalsceneexit_main)


    .status(Main , *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,           finalvisscene2_main)
    
    .status(Exec , *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY,            vissceneentry_exec)
    .status(Exec , *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT,      vissceneattackwait_exec)


    .game_acmd          ("game_final",                          game_final,                         Priority::Low)
    .sound_acmd         ("sound_final",                         sound_final,                        Priority::Low)
    .effect_acmd        ("effect_final",                        effect_final,                       Priority::Low)

    .game_acmd          ("game_finalvisualscene01",             game_finalvisualscene01,            Priority::Low)
    .sound_acmd         ("sound_finalvisualscene01",            sound_finalvisualscene01,           Priority::Low)

    .game_acmd          ("game_finalvisualscene02",             game_finalvisualscene02,            Priority::Low)
    .sound_acmd         ("sound_finalvisualscene02",            sound_finalvisualscene02,           Priority::Low)

    .game_acmd          ("game_finalvisualscene03",             game_finalvisualscene03,            Priority::Low)
    .sound_acmd         ("sound_finalvisualscene03",            sound_finalvisualscene03,           Priority::Low)

    .game_acmd          ("game_fakefinalvisualscene01",         game_fakefinalvisualscene01,        Priority::Low)
    .sound_acmd         ("sound_fakefinalvisualscene01",        sound_fakefinalvisualscene01,       Priority::Low)
    .effect_acmd        ("effect_finalvisualscene02",           effect_finalvisualscene02,          Priority::Low)

    .game_acmd          ("game_heavenlydashloop",               game_finaldash,                     Priority::Low)

    .game_acmd          ("game_heavenlydashattack",             game_finaldashattack,               Priority::Low)

    .game_acmd          ("game_finalend",                       game_finalend,                      Priority::Low)
    .game_acmd          ("game_finalairend",                    game_finalend,                      Priority::Low)
    .sound_acmd         ("sound_finalend",                      sound_finalend,                     Priority::Low)
    .sound_acmd         ("sound_finalairend",                   sound_finalend,                     Priority::Low)
    .effect_acmd        ("effect_finalend",                     effect_finalend,                    Priority::Low)
    .effect_acmd        ("effect_finalairend",                  effect_finalend,                    Priority::Low)

    .game_acmd          ("game_heavenlystart",                  game_heavenlystart,                 Priority::Low)

    //.status(Init, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK, finalvisscene2_init)
    //.status(Init, *FIGHTER_STATUS_KIND_FINAL, finalvisscene2_init)

    //.status(Main, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END, finalvissceneend_main)

/* 

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_main)
    //.status(Exit, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_exit)
    .status(Init, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_init)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_exec)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_end)
    */
    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY, vissceneentry_pre)
    //.status(Init, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY, vissceneentry_init)

    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT, vissceneattackwait_pre)
    //.status(Init, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT, vissceneattackwait_init)









    //.status(Main, *FIGHTER_STATUS_KIND_DEMO, win_pre)
    //.status(Pre, *FIGHTER_STATUS_KIND_DEMO, win_pre)
    //.status(Main,*FIGHTER_STATUS_KIND_DEMO, win_main)
    //.status(Exit,*FIGHTER_STATUS_KIND_DEMO, win_exit)
    //.status(Init,*FIGHTER_STATUS_KIND_DEMO, win_init)
    //.status(Exec,*FIGHTER_STATUS_KIND_DEMO, win_exec)
    //.status(End, *FIGHTER_STATUS_KIND_DEMO, win_end)
    //.status(Pre, FIGHTER_KAMUI_STATUS_KIND_WIN, win_new_pre)
    //.status(Main, FIGHTER_KAMUI_STATUS_KIND_WIN, win_new_main)
    
    .install();

    Agent::new("kamui_waterdragon").set_costume(costume.to_vec())
    //special_lw
    .game_acmd("game_speciallwhit", special_lw_hit, Priority::Low)
    .effect_acmd("effect_speciallwhit", effect_special_lw_hit, Priority::Low)
    .sound_acmd("sound_speciallwhit", sound_special_lw_hit, Priority::Low)

    .game_acmd("game_specialairlw", special_lw_hit, Priority::Low)
    .effect_acmd("effect_specialairlw", effect_special_lw_hit, Priority::Low)
    .sound_acmd("sound_specialairlw", sound_special_lw_hit, Priority::Low)

    
    //special_hi
    .game_acmd("game_specialhi", special_dragon_hi, Priority::Low)
    .game_acmd("game_entryl", special_hi_start_dragon, Priority::Low)
    .game_acmd("game_entryr", special_dragon_hi_loop, Priority::Low)
    .game_acmd("game_win3", special_dragon_hi_release, Priority::Low)
    .game_acmd("game_entryl", entry_lr_throw_hi, Priority::Low)
    .game_acmd("game_throwhi", entry_lr_throw_hi, Priority::Low)
    .effect_acmd("effect_entryl", effect_entry_lr_throw_hi, Priority::Low)
    .effect_acmd("effect_entryr", effect_entry_lr_throw_hi, Priority::Low)
    .effect_acmd("effect_throwhi", effect_entry_lr_throw_hi, Priority::Low)
    





    .install();


    Agent::new("kamui_dragonhand").set_costume(costume.to_vec())
    .game_acmd("game_dhspecialnendmax", special_lw_hit, Priority::Low)
    .effect_acmd("effect_dhspecialnendmax", effect_special_lw_hit, Priority::Low)
    .sound_acmd("sound_dhspecialnendmax", sound_special_lw_hit, Priority::Low)
    .install();



    Agent::new("kamui_spearhand").set_costume(costume.to_vec())
    .game_acmd("game_attacks4", spear_attacks4, Priority::Low)
    .effect_acmd("effect_attacks4charge", effect_attacks4_charge_spear, Priority::Low)
    .install();

    Agent::new("kamui_ryusensya").set_costume(costume.to_vec())
    .status(Main,*WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_REGULAR, fly_main)
    .status(Pre, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_pre)
    .status(Main,WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_main)
    //.status(Exit,WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_exit)
    //.status(Init,WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_init)
    //.status(Exec,WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_exec)
    //.status(End, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_end)
    .game_acmd("game_regular", game_regular, Priority::Low)
    .effect_acmd("effect_regular", effect_regular, Priority::Low)
    .game_acmd("game_shotmax", game_shotmax, Priority::Low)
    .effect_acmd("effect_shotmax", effect_shotmax, Priority::Low)
    .sound_acmd("sound_shotmax", sound_shotmax, Priority::Low)

    .install();


    Agent::new("fighter")
    .on_line(Main, opff_global)
    .on_start(agent_init_global)


    //.status(Pre, FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_pre)
    //.status(Main,FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_main)
    //.status(Exit,FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_exit)
    //.status(Init,FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_init)
    //.status(Exec,FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_exec)
    //.status(End, FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_upb_end)

    
    .status(Main, *FIGHTER_STATUS_KIND_THROWN, thrown_status_common)
    .status(End, *FIGHTER_STATUS_KIND_THROWN, thrown_status_common_end)


    //.game_acmd("game_catchedpotspecialhi", game_catchedpotspecialhi, Priority::Low)


    //.status(Pre, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_upb_pre)
    //.status(Main, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_final_main)
    //.status(End, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_final_end)
    //.status(Init, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_upb_init)
    //.status(Exec, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_upb_exec)
    //.status(End, FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_upb_end)


    .install();


    skyline::install_hooks!(
        kamui_link_event,
        noping_the_eff,
        //get_param_float_hook,
        handle_draw_potemkin,
        autoturn_situation_check_hook,
    );
    unsafe {
        KAMUI_VTABLE_ENTRIES[0] = 0x4FC8B50;
        for i in 1..146 {
            KAMUI_VTABLE_ENTRIES[i] = KAMUI_VTABLE_ENTRIES[0] + (0x8 * (i as usize));
        }

        KAMUI_WEAPON_VTABLE_ENTRIES[0] = 0x4fc9b50; //INSERT WEAPON VTABLE START OFFSET
        for i in 1..102 {
            KAMUI_WEAPON_VTABLE_ENTRIES[i] = KAMUI_WEAPON_VTABLE_ENTRIES[0] + (0x8 * (i as usize));
        }
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_FRAME_3],kamui_on_frame as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_SEARCH],kamui_on_search as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::RESET_1], kamui_reset as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::INITIALIZE], kamui_init as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_SHIELD_COLLISION], kamui_on_shield as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_DAMAGE], kamui_on_damage as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_ATTACK], kamui_on_attack as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_CHANGE_MOTION], kamui_on_change_motion as u64);

    }

    skyline::patching::Patch::in_text(0x3414534).nop(); // nops the waterball effect
    skyline::patching::Patch::in_text(0xb616d0).nop(); // nops the set pos at the end of final smash
    skyline::patching::Patch::in_text(0xb5ccc0).nop().expect("Failed to patch 0xb5ccc0"); // nops the sword effect
}