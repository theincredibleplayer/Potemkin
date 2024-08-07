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


pub unsafe extern "C" fn caught_upb_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter);
}
pub unsafe extern "C" fn caught_upb_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter);
}
pub unsafe extern "C" fn caught_upb_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter);
}
pub unsafe extern "C" fn caught_upb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter);
}
pub unsafe extern "C" fn caught_upb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter);
}

pub unsafe extern "C" fn CaptureWaitCommon(fighter: &mut L2CFighterCommon) -> bool {
    /*if ( ( ControlModule::get_clatter_time(fighter.module_accessor,0) <= 0.0 ) == false) {
        return false;
    }*/
    if ( ( WorkModule::is_flag(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP) != true)) {
        if ( ( fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR ) ) {
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
            return true;
        }
        if ((WorkModule::is_enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_CUT) != true)){
            return false;
        }
        //L2CFighterBase::change_status((L2CFighterBase *)this,(L2CValue)&LStack_90,(L2CValue)&LStack_a 0);
        fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_CUT.into(), false.into());
    } else {
        //L2CFighterBase::change_status((L2CFighterBase *)this,(L2CValue)&LStack_70,(L2CValue)&LStack_8 0);
        fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
    }
    return true;
}

pub unsafe extern "C" fn status_CaptureWait_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let the_motion = 0xccfd2e2d2;
    if ( ( WorkModule::get_int(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_SITUATION) == fighter.global_table[0x16] ) == true) {
        if ((fighter.global_table[0x16] != *SITUATION_KIND_GROUND)) {
          GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
          if (WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_BUTTERFLYNET)) {
            let the_motion = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_MOTION_KIND_GROUND);
          }
          else {
            if (WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_PICKEL)) {
              let the_motion = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_MOTION_KIND_GROUND);
            }
            else {
              let the_motion = CaptureModule::motion(fighter.module_accessor,Hash40::new_raw(0xccfd2e2d2),WorkModule::get_int(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_MOTION_KIND_OFFSET));
            }
          }
          WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_FLAG_AIR);
        }
        else {
          fighter.FighterStatusCaptrue_set_correct_ground();
          let the_motion = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_MOTION_KIND_GROUND);
        }
        if ((WorkModule::is_flag(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_FLAG_FIRST) != true)) {
          MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(the_motion),0.0,1.0,false,0.0,false,false);
          WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_FLAG_FIRST);
        }
        else {
          MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new_raw(the_motion),-1.0,1.0,0.0,false,false);
        }
        WorkModule::set_int(fighter.module_accessor,fighter.global_table[0x16].get_i32(),*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_SITUATION);
      }    
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn caught_upb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    status_CaptureWait_loop(fighter);
    let some_bool = false;//CaptureWaitCommon(fighter);
    if (some_bool == true) {
        fighter.sub_capture_wait_uniq_process_fix_pos();
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(caught_upb_main_loop as *const () as _))
}



pub unsafe extern "C" fn caught_upb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_REACTION_VALUE as u8}, 30.0, -1.0, -1);
    return status_CaptureWait_loop(fighter);
}




pub unsafe extern "C" fn thrown_status_common(fighter: &mut L2CFighterCommon) -> L2CValue {
  let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
  let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
  println!("caught status hook successful probably");
  if (StatusModule::status_kind(parent_boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
  || StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START)
  && (utility::get_kind(&mut *parent_boma) == *FIGHTER_KIND_KAMUI) {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
    let mut motion = hash40("catched_pot_special_hi");
    let mut share_type = 0;
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        motion = FighterMotionModuleImpl::add_body_type_hash(
          fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
        motion = FighterMotionModuleImpl::add_body_type_hash(
          fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_BIG
        );
    }
    WorkModule::set_int64(fighter.module_accessor, motion as i64,*FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);

  }
  return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}

pub unsafe extern "C" fn game_catchedpotspecialhi(agent: &mut L2CAgentBase) {
  if macros::is_excute(agent) {
    MotionModule::set_flip(agent.module_accessor,true,true,false);
    println!("hiiiiiiiiiiiiiiiiii");
  }
}