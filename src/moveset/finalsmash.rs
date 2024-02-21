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
  if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
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
    if ((fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND) != false) {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(Final_main_loop as *const () as _))
  }
}


pub unsafe extern "C" fn Final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  let module_accessor = fighter.module_accessor;
  if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL) {
  if CancelModule::is_enable_cancel(module_accessor)
  && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
  && fighter.sub_air_check_fall_common().get_bool() {
      return L2CValue::I32(0)
  }
  if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
  }
  else {
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
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
      fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(),false.into());
    }
    else {
      fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
  }
  else {
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS) {
      fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
      return L2CValue::I32(0)
    }
  }
  return L2CValue::I32(0) 
  }else{
  if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM){
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
}



pub unsafe extern "C" fn Final_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
  let module_accessor = fighter.module_accessor;
  ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
  return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
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
    JostleModule::set_status(fighter.module_accessor, false);
    motion_setup(fighter,false);
    let module_accessor = fighter.module_accessor;
    //KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    //GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
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
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,10.0,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor, timer + 1.0 ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,  10.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP.into(),false.into());
    }
    if (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == true)
    && (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 20.0) {
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
        return L2CValue::I32(0)
    }
    if (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 120.0) {
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
        return L2CValue::I32(0)
    }
    if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
      fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
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
  if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM){
    KineticModule::clear_speed_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM);
    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
  }
  if MotionModule::is_end(fighter.module_accessor) {
    KineticModule::clear_speed_all(fighter.module_accessor);
    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
  }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn finaldashattack_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
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

pub unsafe extern "C" fn game_finaldash(agent: &mut L2CAgentBase) {
  if macros::is_excute(agent) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
}
}

pub unsafe extern "C" fn game_finaldashattack(agent: &mut L2CAgentBase) {
  if macros::is_excute(agent) {
    macros::SLOW_OPPONENT(agent, 400.0, 7.0);
}
  frame(agent.lua_state_agent, 7.0);
  if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 30, 100, 180, 0, 20.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 100.0, false);
  }
  frame(agent.lua_state_agent, 14.0);
  if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
  }
}

pub unsafe extern "C" fn finalsceneexit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END)(fighter);
}

pub unsafe extern "C" fn finalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  KineticModule::clear_speed_all(fighter.module_accessor);
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_u"),0.0,1.0,false,0.0,false,false);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    LinkModule::remove_model_constraint(enemy,true);
    if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
        LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("hip"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_SCALE as u32,true);
      }
      let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
      AreaModule::set_whole(fighter.module_accessor, false);
      WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_DOWN);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_visual_scene_01"),0.0,1.0,false,0.0,false,false);
      ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
      ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
      ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
      smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
      fighter.sub_shift_status_main(L2CValue::Ptr(finalsceneenter_main_loop as *const () as _))

}

unsafe fn finalsceneenter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
      if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK.into(),false.into());
      }
      return L2CValue::I32(0)
}

pub unsafe extern "C" fn finalsceneattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
      let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
      AreaModule::set_whole(fighter.module_accessor, false);
      WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
      MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_visual_scene_02"),0.0,1.0,false,0.0,false,false);
      smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      fighter.sub_shift_status_main(L2CValue::Ptr(finalsceneattack_main_loop as *const () as _))

}

unsafe fn finalsceneattack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
      if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT.into(),false.into());
      }
      return L2CValue::I32(0)
}

pub unsafe extern "C" fn fakefinalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  KineticModule::clear_speed_all(fighter.module_accessor);
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_u"),0.0,1.0,false,0.0,false,false);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    LinkModule::remove_model_constraint(enemy,true);
    if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
        LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("hip"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_SCALE as u32,true);
      }
      let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
      AreaModule::set_whole(fighter.module_accessor, false);
      WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_DOWN);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_visual_scene_01"),0.0,1.0,false,0.0,false,false);
      ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
      ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
      ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
    //smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
      fighter.sub_shift_status_main(L2CValue::Ptr(fakefinalsceneenter_main_loop as *const () as _))

}

unsafe fn fakefinalsceneenter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
      if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
      }
      return L2CValue::I32(0)
}
pub unsafe extern "C" fn fakefinalsceneenter_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
  PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
  return smashline::original_status(Pre, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}
pub unsafe extern "C" fn fakefinalsceneenter_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exit, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}
pub unsafe extern "C" fn fakefinalsceneenter_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Exec, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}
pub unsafe extern "C" fn fakefinalsceneenter_init(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(Init, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}
pub unsafe extern "C" fn fakefinalsceneenter_end(fighter: &mut L2CFighterCommon) -> L2CValue {
  return smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}


pub unsafe extern "C" fn game_finalend(agent: &mut L2CAgentBase) {
 if macros::is_excute(agent){
  let module_accessor = agent.module_accessor;
  let enemy_id = WorkModule::get_int(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
  let enemy = sv_battle_object::module_accessor(enemy_id as u32);
  
  PostureModule::set_pos_2d(agent.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
  WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  LinkModule::remove_model_constraint(enemy,true);
  if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
      LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
  }
  let pos = Vector2f{x: PostureModule::pos_x(module_accessor) + (20.0 * PostureModule::lr(module_accessor)), y: PostureModule::pos_y(module_accessor) + 30.0};
  PostureModule::set_pos_2d(enemy,&pos);

  macros::CAM_ZOOM_OUT(agent);
  smash_script::camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
  WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
  WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_DOWN);
  WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalvisualscene03.nuanmb"), false, false);
 }
frame(agent.lua_state_agent, 45.0);
  if macros::is_excute(agent) {
      macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_KAMUI_FINAL, 0, 13.0, 60, 125, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
      WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
  }
  frame(agent.lua_state_agent, 50.0);
  if macros::is_excute(agent) {
      WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
  }
}


pub unsafe extern "C" fn finaljump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  KineticModule::clear_speed_all(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_start"),0.0,1.0,false,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(finaljump_main_loop as *const () as _))

}

unsafe fn finaljump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  if fighter.global_table[0xe].get_f32() >= 12.0 {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,2.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,2.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 2.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }
    if MotionModule::is_end(fighter.module_accessor) {
      fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE.into(),false.into());
    }
    return L2CValue::I32(0)
}