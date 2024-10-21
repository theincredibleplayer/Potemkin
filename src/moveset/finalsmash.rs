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





unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("final"),-1.0,1.0,0.0,false,false);
        } 
    } else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("final"),-1.0,1.0,0.0,false,false);
        }
    }
}

pub unsafe extern "C" fn finalvisscene2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter,"catched_pot_final_scene_02", 1.0);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK)(fighter);
}


pub unsafe extern "C" fn Final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL) {
        WorkModule::set_float(fighter.module_accessor, 0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1); 
        AreaModule::set_whole(fighter.module_accessor,false);
        KineticModule::clear_speed_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DISABLE_OUTSIDE_ENERGY);
        if ((fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND) != false) {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false); //should be air
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false); //ground
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ZOOM_START);
        fighter.sub_shift_status_main(L2CValue::Ptr(Final_main_loop as *const () as _))


    } else {
        KineticModule::clear_speed_all(fighter.module_accessor);
        let module_accessor = fighter.module_accessor;
        AreaModule::set_whole(fighter.module_accessor, false);

        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        AreaModule::set_whole(fighter.module_accessor, false);
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene01.nuanmb"), false, false);
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("fake_final_visual_scene_01"),0.0,1.0,false,0.0,false,false);

        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("fake_final_visual_scene_01"), false, -1.0);
        let plane = ArticleModule::get_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);

        
        fighter.sub_shift_status_main(L2CValue::Ptr(Final_main_loop as *const () as _))
    }
}


pub unsafe extern "C" fn Final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL) {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        } else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }


        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL);
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP.into(),false.into());
            return L2CValue::I32(0)
        }


        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(),false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        } else {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS) {
                fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
                return L2CValue::I32(0)
            }
        }
        return L2CValue::I32(0) 
    }else{
        let module_accessor = fighter.module_accessor;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        AreaModule::set_whole(fighter.module_accessor, false);


        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
        }
        return L2CValue::I32(0)
    }
}



pub unsafe extern "C" fn Final_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return L2CValue::I32(0)
}






pub unsafe extern "C" fn finaldashloop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_FINAL) as u32,
        0);
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn finaldashloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    motion_setup(fighter,false);
    let module_accessor = fighter.module_accessor;
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_dash_loop"),0.0,1.0,false,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(finaldashloop_main_loop as *const () as _))
}

unsafe fn finaldashloop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut current_speed = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
    let module_accessor = fighter.module_accessor;
    let timer = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,7.0,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor, timer + 1.0 ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_NONE,  5.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP.into(),false.into());
    }
    if (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == true)
    && (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 20.0) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK.into(),false.into());
        return L2CValue::I32(0)
    }
    if (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 60.0) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK.into(),false.into());
        return L2CValue::I32(0)
    }
    if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK.into(),false.into());
        return L2CValue::I32(0)
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn finaldashloop_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}
pub unsafe extern "C" fn finaldashloop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}
pub unsafe extern "C" fn finaldashloop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}
pub unsafe extern "C" fn finaldashattack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}


pub unsafe extern "C" fn finaldashattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  let module_accessor = fighter.module_accessor;
  KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
  KineticModule::clear_speed_all(fighter.module_accessor);
  KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
  GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
  fighter.clear_lua_stack();
  lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
  sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
  MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_dash_attack"),0.0,1.0,false,0.0,false,false);
  WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    fighter.sub_shift_status_main(L2CValue::Ptr(finaldashattack_main_loop as *const () as _))
}

unsafe fn finaldashattack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM)
    }
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM)
    && WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP){
    KineticModule::clear_speed_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM);
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP.into(),false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn finaldashattack_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS){
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        JostleModule::set_status(fighter.module_accessor, false);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_genesis_bg"), false, true);
        return L2CValue::I32(0)
    }else{
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
    }
}
pub unsafe extern "C" fn finaldashattack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}
pub unsafe extern "C" fn finaldashattack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}
pub unsafe extern "C" fn finaldashattack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
}

pub unsafe extern "C" fn finalsceneexit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    grabbed_anim_selector(fighter,"catched_pot_final_scene_03", 1.0);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene03.nuanmb"), false,false);
    PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END)(fighter);
}

pub unsafe extern "C" fn finalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}

pub unsafe extern "C" fn finalsceneattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK)(fighter);
}




pub unsafe extern "C" fn finaljump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_start"),0.0,1.0,false,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(finaljump_main_loop as *const () as _))
}

unsafe fn finaljump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;

    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP)  {
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,10.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,10.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,    3.0   ,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 10.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }
    if MotionModule::is_end(module_accessor) {
        WorkModule::on_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL);
        WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn finaljump_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS){
        JostleModule::set_status(fighter.module_accessor, false);
        return L2CValue::I32(0)
    }else{
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
    }
}

pub unsafe extern "C" fn vissceneentry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("entry pre");
    macros::CAM_ZOOM_OUT(fighter);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x2c49254dfb));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x2c1b582d81));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);


    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK);
    return 1.into()
}

pub unsafe extern "C" fn vissceneentry_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("entry init");
    return smashline::original_status(Init, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}

pub unsafe extern "C" fn vissceneattackwait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("wait pre");
    macros::CAM_ZOOM_OUT(fighter);
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x327bfca840));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x3a2bb2496d));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x322981c83a));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END);
    return 1.into()
}

pub unsafe extern "C" fn vissceneattackwait_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("wait init");

    return smashline::original_status(Init, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT)(fighter);

}