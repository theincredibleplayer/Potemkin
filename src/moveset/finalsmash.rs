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
    KineticModule::clear_speed_all(fighter.module_accessor);
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
    AreaModule::set_whole(fighter.module_accessor, false);
    AreaModule::set_whole(enemy, false);
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_u"),0.0,1.0,false,0.0,false,false);
    LinkModule::remove_model_constraint(enemy,true);
    if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
        LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("hip"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_DISABLE_RESTRICT_Z as u32,true);
      }
      HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
      HitModule::set_whole(enemy, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
      let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
      AreaModule::set_whole(fighter.module_accessor, false);
      WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene01.nuanmb"), false, false);
      MotionModule::change_motion(fighter.module_accessor,Hash40::new("fake_final_visual_scene_01"),0.0,1.0,false,0.0,false,false);

      //let plane_id = ArticleModule::get_active_num(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
      ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
      ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
      ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("fake_final_visual_scene_01"), false, -1.0);
      let plane = ArticleModule::get_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
      let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
      LinkModule::remove_model_constraint(plane_boma,true);
      LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
      //let offset_rot = Vector3f{x: 0.0, y: 0.0, z: -90.0};
      //LinkModule::set_constraint_rot_offset(plane_boma,&offset_rot);
      //let offset_pos = Vector3f{x: 0.0, y: 12.0, z: 0.0};
      //LinkModule::set_constraint_translate_offset(plane_boma,&offset_pos);

      
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
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::off_flag(enemy, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    AreaModule::set_whole(fighter.module_accessor, false);
    AreaModule::set_whole(enemy, false);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
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
    //JostleModule::set_status(fighter.module_accessor, false);
    motion_setup(fighter,false);
    let module_accessor = fighter.module_accessor;
    //KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    //GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    //fighter.clear_lua_stack();
    //lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
    //sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
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
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,5.0,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor, timer + 1.0 ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_NONE,  4.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

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
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK.into(),false.into());
        return L2CValue::I32(0)
    }
    if (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 120.0) {
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
    JostleModule::set_status(agent.module_accessor, false);
    HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 17.0, 13.0, 8.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
}
}

pub unsafe extern "C" fn game_finaldashattack(agent: &mut L2CAgentBase) {
  if macros::is_excute(agent) {
    JostleModule::set_status(agent.module_accessor, false);
    macros::SLOW_OPPONENT(agent, 400.0, 7.0);
    HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
}
  frame(agent.lua_state_agent, 7.0);
  if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 368, 100, 180, 0, 19.0, 13.0, 8.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 30.0, y: 50.0}, 40, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
  }
  frame(agent.lua_state_agent, 14.0);
  if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
  }
  frame(agent.lua_state_agent, 28.0);
  if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP);
  }
}

pub unsafe extern "C" fn finalsceneexit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  let module_accessor = fighter.module_accessor;
  let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
  let enemy = sv_battle_object::module_accessor(enemy_id as u32);
  WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);

  LinkModule::remove_model_constraint(enemy,true);
  if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
      LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
  }
  macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene03.nuanmb"), false,false);

  StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
  StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_KAMUI_FINAL_TARGET_DAMAGE, false);
  MotionModule::change_motion(enemy,Hash40::new("fall_damage"),0.0,1.0,false,0.0,false,false);
  LinkModule::remove_model_constraint(enemy,true);
  if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
      LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
      LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("hip"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_DISABLE_RESTRICT_Z as u32,true);
    }
  PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
  return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END)(fighter);
}

pub unsafe extern "C" fn finalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  KineticModule::clear_speed_all(fighter.module_accessor);
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);


      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);

      
      LinkModule::remove_model_constraint(enemy,true);
      if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
          LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
      }
      let pos = Vector2f{x: 0.0, y: 0.0};
      PostureModule::set_pos_2d(enemy,&pos);    
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_KAMUI_FINAL_TARGET_DAMAGE, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_u"),0.0,1.0,false,0.0,false,false);
    LinkModule::remove_model_constraint(enemy,true);
    if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
        LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("trans"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_DISABLE_RESTRICT_Z  as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
        let offset_rot = Vector3f{x: 0.0, y: -90.0, z: 0.0};
        LinkModule::set_constraint_rot_offset(enemy,&offset_rot);
      }
      return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);

}

pub unsafe extern "C" fn finalsceneattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK)(fighter);
}


pub unsafe extern "C" fn fakefinalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  let module_accessor = fighter.module_accessor;
  PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
  KineticModule::clear_speed_all(fighter.module_accessor);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    //StatusModule::set_situation_kind(enemy, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
    //GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    //StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_u"),0.0,1.0,false,0.0,false,false);
    //GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    LinkModule::remove_model_constraint(enemy,true);
    if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) == false {
        LinkModule::link(enemy,*LINK_NO_CAPTURE,(*(fighter.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(enemy,*LINK_NO_CAPTURE,Hash40::new("hip"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_DISABLE_RESTRICT_Z as u32,true);
      }
      let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
      AreaModule::set_whole(fighter.module_accessor, false);
      WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
      MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_visual_scene_01"),0.0,1.0,false,0.0,false,false);
      ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
      ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
      ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
    //smash::app::lua_bind::FighterManager::start_finalbg(fighter_manager,*FIGHTER_KIND_KAMUI,1.0);
      fighter.sub_shift_status_main(L2CValue::Ptr(fakefinalsceneenter_main_loop as *const () as _))

}
unsafe fn fakefinalsceneenter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  //KineticModule::clear_speed_all(fighter.module_accessor);
  let module_accessor = fighter.module_accessor;
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
  KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
  PostureModule::set_pos_2d(fighter.module_accessor,&Vector2f{x: 0.0 , y: 0.0});
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
  let module_accessor = agent.module_accessor;
  let enemy_id = WorkModule::get_int(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
  let enemy = sv_battle_object::module_accessor(enemy_id as u32);
 if macros::is_excute(agent){
  HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
  HitModule::set_whole(enemy, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
  WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
  //macros::REQ_FINAL_START_CAMERA(agent,Hash40::new("d04finalvisualscene03.nuanmb"), false);
  ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
 }
  frame(agent.lua_state_agent, 49.0);
  if macros::is_excute(agent) {
    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    HitModule::set_whole(enemy, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    
  }
  frame(agent.lua_state_agent, 53.0);
  if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 35, 50, 0, 50, 25, 0.0, 10.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
    AttackModule::set_no_dead_all(agent.module_accessor, true, true);
    AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
    WorkModule::off_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    WorkModule::off_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    //CameraModule::zoom_out(agent.module_accessor, 0);
    //macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_FALL, false);
    MotionModule::change_motion(enemy,Hash40::new("down_wait_d"),0.0,1.0,false,0.0,false,false);
  }
  wait(agent.lua_state_agent, 5.0);
  if macros::is_excute(agent) {
    //KineticModule::clear_speed_all(enemy);
    AttackModule::clear_all(agent.module_accessor);
  }
  frame(agent.lua_state_agent, 108.0);
  if macros::is_excute(agent) {
    //CameraModule::zoom_out(agent.module_accessor, 0);
  }
  frame(agent.lua_state_agent, 127.0);
  if macros::is_excute(agent) {
    StatusModule::change_status_request(enemy, *FIGHTER_STATUS_KIND_FALL, false);
      LinkModule::remove_model_constraint(enemy,true);
      if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
          LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
      }
      if PostureModule::lr(agent.module_accessor) < 0.0{
        let pos = Vector2f{x: -15.0, y: 10.0};
        PostureModule::set_pos_2d(enemy,&pos);
        let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        PostureModule::set_rot(enemy,&rot,0);
        PostureModule::set_lr(enemy, -1.0);
      }else{
        let pos = Vector2f{x: 15.0, y: 10.0};
        PostureModule::set_pos_2d(enemy,&pos);
        let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        PostureModule::set_rot(enemy,&rot,0);
      }
    }
  if DamageModule::damage(enemy, 0) > 100.0 {
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
      macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 20.0, 50, 0, 200, 100, 25, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
      //AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 50.0, y: 15.0}, 40, false);
      AttackModule::set_no_dead_all(agent.module_accessor, true, false);
      AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 100.0, false);
      macros::FT_MOTION_RATE(agent, 2.0);
      //macros::EFFECT_FOLLOW(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
      EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
      AttackModule::clear_all(agent.module_accessor);
    }
    for n in 1..21{
      if macros::is_excute(agent) {
        //SlowModule::clear_whole(agent.module_accessor);
        if n<=3{
        EffectModule::req_follow(enemy, Hash40::new("sys_explosion_sign"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        macros::LAST_EFFECT_SET_RATE(agent, 3.0);
        }
        SlowModule::set_whole(agent.module_accessor, 3, 100);
      wait(agent.lua_state_agent, 1.0);
      }
    }
    frame(agent.lua_state_agent, 152.0);
    if macros::is_excute(agent) {
      macros::FT_MOTION_RATE(agent, 1.0);
      StatusModule::change_status_request_from_script(enemy, FIGHTER_STATUS_KIND_DEAD.into(), false.into());
      WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
    }
    frame(agent.lua_state_agent, 155.0);
    if macros::is_excute(agent) {
      SlowModule::clear_whole(agent.module_accessor);
    }
  } else {
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
      macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 20.0, 35, 50, 0, 50, 25, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
      AttackModule::clear_all(agent.module_accessor);
    }
  }
}

pub unsafe extern "C" fn sound_finalend(agent: &mut L2CAgentBase) {
  frame(agent.lua_state_agent, 5.0);
  if macros::is_excute(agent) {
      //macros::PLAY_SE(agent, Hash40::new("heavenly_1_4"));
  }
  frame(agent.lua_state_agent, 53.0);
  if macros::is_excute(agent) {
      macros::PLAY_SE(agent, Hash40::new("se_kamui_final02"));
  }
}

pub unsafe extern "C" fn effect_finalend(agent: &mut L2CAgentBase) {
  for _ in 0..26 {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("haver"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_soccer"), Hash40::new("havel"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
      }
    wait(agent.lua_state_agent, 2.0);
    }
  frame(agent.lua_state_agent, 53.0);
  if macros::is_excute(agent) {
    macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, 7, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
    macros::LAST_EFFECT_SET_RATE(agent, 0.85);
    macros::EFFECT(agent, Hash40::new("tiki_breath_embers"), Hash40::new("top"), 0, 0, -5, 0, 90, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
    macros::LAST_EFFECT_SET_RATE(agent, 0.85);
    macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 0, -20, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, false);
    macros::LAST_EFFECT_SET_RATE(agent, 0.3);
  }
  frame(agent.lua_state_agent, 110.0);
  if macros::is_excute(agent) {
    EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_soccer"), false, true);
  }
}

pub unsafe extern "C" fn finaljump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  KineticModule::clear_speed_all(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_start"),0.0,1.0,false,0.0,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(finaljump_main_loop as *const () as _))

}

unsafe fn finaljump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  //KineticModule::clear_speed_all(fighter.module_accessor);
  if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP)  {
    let module_accessor = fighter.module_accessor;
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
    if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
      WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL);
      WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
      fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
      return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
      WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL);
      WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
      fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn game_heavenlystart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
      HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
      JostleModule::set_status(agent.module_accessor, false);
      ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
  }
  frame(agent.lua_state_agent, 7.0);
  if macros::is_excute(agent) {
      macros::PLAY_SE(agent, Hash40::new("se_kamui_final04"));
      macros::FT_MOTION_RATE(agent, 6.0);
      macros::SLOW_OPPONENT(agent, 400.0, 30.0);
      //stub_kill_screen();
      //camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 50, 50);
      //CameraModule::add_camera_range_offset(agent.module_accessor,&Vector3f{x:30.0,y:30.0,z:5.0}, 0);
      macros::CAM_ZOOM_IN_arg5(
      agent,
      /*frames to zoom in totally*/5.0,
      /*idk best kept 0*/0,
      /*the amt of zoom needed, higher the number lower the zoom*/1.5,
      /*y_rot*/0.0,
      /*x_rot*/0.0,
      );
      //CameraModule::update_force(agent.module_accessor,*MA_MSC_CMD_CAMERA_CAM_OFFSET,&Vector3f{x:30.0,y:30.0,z:5.0},&Vector3f{x:30.0,y:30.0,z:5.0});
      let pos = Vector2f{
      x:0.0,
      y:100.0,
      };
      //CameraModule::zoom_in(agent.module_accessor, 1, 1, 1.0,  &pos, true);
      //CameraModule::set_camera_range_offset(agent.module_accessor,&Vector3f{x:30.0,y:30.0,z:5.0}, 0);
  }
  frame(agent.lua_state_agent, 12.0);
  if macros::is_excute(agent) {
      macros::FT_MOTION_RATE(agent, 1.0);
      SlowModule::clear_whole(agent.module_accessor);
      WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
      macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 0.0, 0, 0, 0, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
      AttackModule::set_no_dead_all(agent.module_accessor, true, false);
      AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
      AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
      AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
      AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
      CameraModule::zoom_out(agent.module_accessor, 30);
  }
}

pub unsafe extern "C" fn vissceneentry_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
  CameraModule::zoom_out(fighter.module_accessor, 0);
  fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK.into(),false.into());
  return L2CValue::I32(0)
}


pub unsafe extern "C" fn vissceneattackwait_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
  CameraModule::zoom_out(fighter.module_accessor, 0);
  fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_END.into(),false.into());
  return L2CValue::I32(0)
}
