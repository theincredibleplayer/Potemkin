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
use smash_script::{*, macros::*};

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;

use crate::moveset::*;

pub unsafe extern "C" fn roman_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_NONE) as u64,
        (0) as u32,
        (0) as u32,
        0);
    return L2CValue::I32(0)
}



pub unsafe extern "C" fn roman_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        //GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("roman_cancel_air"),0.0,1.0,false,0.0,false,false);
    }
    else {
        //GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("roman_cancel"),0.0,1.0,false,0.0,false,false);
    }
    KineticModule::clear_speed_all(module_accessor);
    WorkModule::set_float(module_accessor, 0.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_ROMAN_CANCEL_METER_DRAIN);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let mut stick_x = fighter.global_table[STICK_X].get_f64();
    let mut stick_y = fighter.global_table[STICK_Y].get_f64();
    let mut moving_x : f64 = 0.0;
    let mut moving_y : f64 = 0.0;
    let limiting_speed = 5.0;
    let speed_mul = 0.5;

    if !WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
        if stick_y.abs() > stick_x.abs(){
            if stick_y < 0.0 {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                StatusModule::set_keep_situation_air(fighter.module_accessor,true);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                moving_y = -1.0;
                moving_x = 0.0;
            }else if stick_y > 0.0{
                fighter.set_situation(SITUATION_KIND_AIR.into());
                StatusModule::set_keep_situation_air(fighter.module_accessor,true);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                moving_y = 1.0;
                moving_x = 0.0;
            } else {
                moving_x = 0.0;
                moving_y = 0.0;
            }
        } else if stick_y.abs() <= stick_x.abs(){
            if stick_x < 0.0 {
                moving_x = -1.0;
                moving_y = 0.0;
            } else if stick_x > 0.0 {
                moving_x = 1.0;
                moving_y = 0.0;
            } else {
                moving_x = 0.0;
                moving_y = 0.0;
            }
        }
    }
    WorkModule::set_float(module_accessor, moving_x as f32, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_X);
    WorkModule::set_float(module_accessor, moving_y as f32, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_Y);
    WorkModule::set_float(module_accessor, limiting_speed as f32, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_LIMITING_SPEED);
    WorkModule::set_float(module_accessor, speed_mul as f32, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_SPEED_MUL);

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);


    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 1.75 * speed_mul * moving_x /*PostureModule::lr(module_accessor)*/,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0, speed_mul * moving_y,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,limiting_speed);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);







    fighter.sub_shift_status_main(L2CValue::Ptr(roman_cancel_main_loop as *const () as _))
}

unsafe fn roman_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let timer = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_ROMAN_CANCEL_METER_DRAIN);
    if timer < 50.0{
        UiManager::add_tension(entry_id,-2.0);
        WorkModule::set_float(module_accessor, timer + 2.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_ROMAN_CANCEL_METER_DRAIN);
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE){
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
        WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);

        //CancelModule::enable_cancel(fighter.module_accessor);
    }


    if //CancelModule::is_enable_cancel(fighter.module_accessor)
     WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE){
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_DETECTED);
            return 0.into();
        }
    }





    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("roman_cancel"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        StatusModule::set_keep_situation_air(fighter.module_accessor,true);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("roman_cancel_air"),-1.0,1.0,0.0,false,false);
        //KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn roman_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    macros::CANCEL_FILL_SCREEN(fighter, 0, 20);
    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);

    if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC){
        EffectModule::detach_kind(module_accessor, Hash40::new("kamui_red_rc_blast"), -1);
    }else if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC){
        EffectModule::detach_kind(module_accessor, Hash40::new("kamui_purple_rc_blast"), -1);
    }else if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
        EffectModule::detach_kind(module_accessor, Hash40::new("kamui_yellow_rc_blast"), -1);
    }else if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC){
        EffectModule::detach_kind(module_accessor, Hash40::new("kamui_blue_rc_blast"), -1);
    }
    
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC);


    if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_DETECTED)
    && fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND{
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        
        let mut moving_x = WorkModule::get_float(module_accessor,  FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_X);
        let mut moving_y = WorkModule::get_float(module_accessor,  FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_Y);
        let limiting_speed= WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_LIMITING_SPEED);
        let speed_mul= WorkModule::get_float(module_accessor,      FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_SPEED_MUL);

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);


        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 4.0 * speed_mul * moving_x /*PostureModule::lr(module_accessor)*/,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0, 4.0 * speed_mul * moving_y,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,limiting_speed);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    } else if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR{
        let direction = PostureModule::lr(module_accessor);
        let speed_x = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_X_SPEED);
        let speed_y = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_Y_SPEED);
        KineticModule::clear_speed_all(module_accessor);
        let speed_vector = smash::phx::Vector3f { x: speed_x * direction, y: speed_y, z: 0.0 };
        KineticModule::add_speed(module_accessor, &speed_vector);
    }



    WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_DETECTED);

    return L2CValue::I32(0)
}
