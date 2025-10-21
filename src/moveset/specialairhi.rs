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

unsafe fn special_air_hi_velocity_and_motion_handler(fighter: &mut L2CFighterCommon, speed_mul: f32, do_motion:bool) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let stick_x = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_X).abs();
    let stick_y = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_Y);
    let degree_range = 30.0;
    let mut speed_scale= 0.0;
    if !fighter.global_table[IS_STOP].get_bool() {
        speed_scale = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED) * speed_mul;
    }else{
        speed_scale = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED) * (speed_mul+0.01);
    }
    let mut speed_x = 0.0;
    let mut speed_y = 0.0;
    let mut angle = stick_y.atan2(stick_x).to_degrees();
    let lr = PostureModule::lr(module_accessor);
    WorkModule::set_float(module_accessor,speed_scale, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED);
    if angle > degree_range {
        speed_x = speed_scale * angle.to_radians().cos();
        speed_y = speed_scale * angle.to_radians().sin();
    }else{
        angle = degree_range;
        speed_x = speed_scale * angle.to_radians().cos();
        speed_y = speed_scale * angle.to_radians().sin();
    }
    if do_motion{
        let frame = MotionModule::frame(module_accessor);
        let rate = MotionModule::rate(module_accessor);
        ArticleModule::add_motion_2nd(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_air_hi_up"),frame,rate,true,(angle - 45.0)/45.0);
        ArticleModule::set_weight(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,1.0-(angle - 45.0)/45.0,true);
        MotionModule::add_motion_2nd(module_accessor, Hash40::new("special_air_hi_up"), frame, rate, true, (angle - 45.0)/45.0);
        MotionModule::set_weight(module_accessor,1.0-(angle - 45.0)/45.0 , true);
    }
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,speed_scale);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_scale);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,speed_y,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, speed_x * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
}



pub unsafe extern "C" fn special_air_hi_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0);
    FighterStatusModuleImpl::set_fighter_status_data
        (fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0);
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn special_air_hi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_hold"),0.0,1.0,false,0.0,false,false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
    
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.1, -0.1);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_hold_ground"),0.0,1.0,false,0.0,false,false);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_hi_hold_main_loop as *const () as _))
}

unsafe fn special_air_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    WorkModule::set_float(module_accessor, fighter.global_table[STICK_X].get_f32(), FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_X);
    WorkModule::set_float(module_accessor, fighter.global_table[STICK_Y].get_f32(), FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_Y);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi_hold_ground"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi_hold"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE.into(),false.into());
    }
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn special_air_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0);
    FighterStatusModuleImpl::set_fighter_status_data
        (fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn special_air_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let lr = PostureModule::lr(module_accessor);
    let stick_x = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_X);
    if (stick_x * lr) < 0.0{
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(module_accessor);
    }
    WorkModule::set_float(module_accessor,3.25, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi"),0.0,1.0,false,0.0,false,false);
    ArticleModule::generate_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_air_hi"),false,-1.0);
    special_air_hi_velocity_and_motion_handler(fighter,1.0,true);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON);
    potemkin_mirror_handler(module_accessor);
    potemkin_weapon_attach_handler(module_accessor);
    JostleModule::set_status(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_hi_main_loop as *const () as _))
}

unsafe fn special_air_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let lr = PostureModule::lr(module_accessor);

    special_air_hi_velocity_and_motion_handler(fighter,0.975,true);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_air_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON);
    ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    JostleModule::set_status(fighter.module_accessor, true);
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_air_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0);
    FighterStatusModuleImpl::set_fighter_status_data
        (fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0);
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_air_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_end"),0.0,1.0,false,0.0,false,false);
    special_air_hi_velocity_and_motion_handler(fighter,1.0,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_hi_end_main_loop as *const () as _))
}

unsafe fn special_air_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
    }


    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
    }
    return L2CValue::I32(0)
}
