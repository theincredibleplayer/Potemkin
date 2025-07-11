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


pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::on_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
    WorkModule::set_int(weapon.module_accessor,90,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor,90,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    MotionModule::change_motion(weapon.module_accessor,Hash40::new("regular"),0.0,1.0,false,0.0,false,false);
  
    let posture = PostureModule::lr(weapon.module_accessor);
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0 * posture, 0.0);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0 * posture, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0, 0.0);
    sv_kinetic_energy::set_limit_speed(weapon.lua_state_agent);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER);
    weapon.fastshift(L2CValue::Ptr(fly_main_loop as *const () as _))
}

pub unsafe extern "C" fn fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //if (WorkModule::get_int(weapon.module_accessor, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER) % 3) == 0{
    //    EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0, 0, 0, 0, 0, false, false);
    //}
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
        weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
    }
    if (WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0) {
        weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
        return L2CValue::I32(0)
    }
    if (WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL)) {
        if ( 2 <= weapon.global_table[0xE].get_int() ) {
            weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
            return L2CValue::I32(0)
        }
    } else {
        if (weapon.sub_ground_module_is_touch_all_consider_speed().get_bool()) {
            if ( 1 <= weapon.global_table[0xE].get_int() ){
                if !WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
                    if (!GroundModule::is_touch(weapon.module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)) {
                        return L2CValue::I32(0)
                    }
                }
                weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
                return L2CValue::I32(0)
            }
            WorkModule::on_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if (weapon.global_table[0xE].get_int() <= 1) {
                weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
                return L2CValue::I32(0)
            }
            //StopModule::set_other_stop(weapon.module_accessor,2,smash::StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
        if (!GroundModule::is_touch(weapon.module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)) {
            return L2CValue::I32(0)
        }
    }
    weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
    return L2CValue::I32(0)   
}

pub unsafe extern "C" fn boom_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return smashline::original_status(Pre, weapon, *WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_REGULAR)(weapon);
}

pub unsafe extern "C" fn boom_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor,Hash40::new("shot_max"),0.0,1.0,false,0.0,false,false);
  

    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_limit_speed(weapon.lua_state_agent);
  
    weapon.fastshift(L2CValue::Ptr(boom_main_loop as *const () as _))
}

pub unsafe extern "C" fn boom_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return L2CValue::I32(0)   
}




//unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
//    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//        if situation_change == false {
//            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_n"),0.0,1.0,false,0.0,false,false);
//            fighter.clear_lua_stack();
//            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
//            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
//        }
//        else {
//            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_n"),-1.0,1.0,0.0,false,false);
//        }
//    }
//    else {
//        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
//        if situation_change == false {
//            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_n"),0.0,1.0,false,0.0,false,false);
//            fighter.clear_lua_stack();
//            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
//            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
//        }
//        else {
//            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_n"),-1.0,1.0,0.0,false,false);
//        }
//    }
//}




pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_n_start"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_n_start"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_n_start"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_n_start"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_n_hold"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_n_hold"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}

unsafe fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if !ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT.into(),false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_n_hold"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_n_hold"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut frame = 0.0;
    if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED){
        frame = 10.0;
        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    }
    println!("frame: {}",frame);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_n_fire"),frame,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_n_fire"),frame,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
}

unsafe fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
    if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED){
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_n_fire"),10.0,1.0,false,0.0,false,false);
        }
        else {
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(module_accessor,Hash40::new("special_n_fire"),10.0,1.0,false,0.0,false,false);
        }
        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    }



    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_n_fire"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_n_fire"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
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



pub unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
    //    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    //}
    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    ShieldModule::clear_all(module_accessor);

    //smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    return L2CValue::I32(0)

}


pub unsafe extern "C" fn special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
    //    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    //}
    ShieldModule::clear_all(module_accessor);

    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    //smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD)(fighter)
    return L2CValue::I32(0)

}


pub unsafe extern "C" fn special_n_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
    //    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    //}
    ShieldModule::clear_all(module_accessor);
    //ShieldModule::clear_all_2
    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    //smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD)(fighter)
    return L2CValue::I32(0)

}