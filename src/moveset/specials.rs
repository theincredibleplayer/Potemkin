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

unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        if stick_x < 1.0 {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F.into(),false.into());
        }else{
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B.into(),false.into());
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_s"),-1.0,1.0,0.0,false,false);
        }
    }
}

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    motion_setup(fighter,false);
    let module_accessor = fighter.module_accessor;
    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
    //KineticModule::add_speed(fighter.module_accessor,&smash::phx::Vector3f { x: 15.0, y: 0.0, z: 0.0 });
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut current_speed = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(fighter.module_accessor,0.5 + current_speed,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL );
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, current_speed * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL.into(),false.into());
    }
    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false
    || GroundModule::is_near_cliff(fighter.module_accessor,PostureModule::pos_x(module_accessor),1.0) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END.into(),false.into());
        return L2CValue::I32(0)
    }
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_slide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_slide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_slide"),0.0,1.0,false,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_slide_loop as *const () as _))
}

unsafe fn side_b_slide_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 3.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let limiting_speed = 5.0;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return L2CValue::I32(0)
    }
    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false
    || GroundModule::is_near_cliff(fighter.module_accessor,PostureModule::pos_x(module_accessor),1.0) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END.into(),false.into());
        return L2CValue::I32(0)
    }
    if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
          fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK.into(),false.into());
          return L2CValue::I32(0)
    }
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_end"),0.0,1.0,false,0.0,false,false);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.7,0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_end_loop as *const () as _))
}

unsafe fn side_b_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    //KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_attack"),0.0,1.0,false,0.0,false,false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_attack_loop as *const () as _))
}

unsafe fn side_b_attack_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_slide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_slide_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn side_b_slide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn game_specialsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 35, 0, 0, 0, 16, 0.0, 13.0, 5.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

pub unsafe extern "C" fn effect_specialsloop(agent: &mut L2CAgentBase) {
    for n in 1..99{
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    wait(agent.lua_state_agent, 30.0);
    }
}
pub unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
}
pub unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 35, 80, 0, 50, 16, 0.0, 14.0, 6.0, Some(0.0), Some(-2.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_buster_dash_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("dolly_buster_dash_ground"), false, true);
    }
}


pub unsafe extern "C" fn megafist_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn megafist_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_mf_f"),0.0,1.0,false,0.0,false,false);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,3.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,3.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 0.8 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.2, -0.2);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    //KineticModule::add_speed(module_accessor, &Vector3f{x:30.0,y:30.0,z:0.0});
    fighter.sub_shift_status_main(L2CValue::Ptr(megafist_f_main_loop as *const () as _))
}

unsafe fn megafist_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 0.8 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    if StatusModule::is_situation_changed(fighter.module_accessor) == true {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING.into(),false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn megafist_f_land_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_s_mf_f_land"),-1.0,1.0,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(megafist_f_land_main_loop as *const () as _))
}


pub unsafe extern "C" fn megafist_f_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn game_specialsmff(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("handr"),15.0,300,80,0,40,6.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,1,0,Hash40::new("handl"),15.0,300,80,0,40,6.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_AIR,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,2,0,Hash40::new("handr"),15.0,75,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,3,0,Hash40::new("handl"),15.0,75,80,0,40,9.0,0.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}



pub unsafe extern "C" fn megafist_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn megafist_b_land_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}