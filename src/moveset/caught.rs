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
        if ( ( fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR ) ) {
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
    if ( ( WorkModule::get_int(fighter.module_accessor,*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_SITUATION) == fighter.global_table[SITUATION_KIND] ) == true) {
        if ((fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND)) {
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
        WorkModule::set_int(fighter.module_accessor,fighter.global_table[SITUATION_KIND].get_i32(),*FIGHTER_STATUS_CAPTURE_WAIT_WORK_INT_SITUATION);
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

    if (StatusModule::status_kind(parent_boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR
    || StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_AIR_THROW)
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

    if (StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF
    || StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER)
    && (utility::get_kind(&mut *parent_boma) == *FIGHTER_KIND_KAMUI) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
        let mut motion = hash40("catched_pot_potemkin_buster");
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

    if (StatusModule::status_kind(parent_boma) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP)
    && (utility::get_kind(&mut *parent_boma) == *FIGHTER_KIND_KAMUI) {
        let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
        let mut motion = hash40("catched_pot_final_scene_01");
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
        let egg_size = WorkModule::get_param_float(fighter.module_accessor, hash40("egg_collision_size"), 0);
        let fighter_kind = utility::get_kind(&mut (*fighter.module_accessor));

        let mut x_offset = 22.0 - height;
        let mut z_offset = (7.2 -egg_size)/4.0;
        let mut y_offset = 0.0;

        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

        let head_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let position = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), hip_pos, true);
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("head"), head_pos, true);
        position.x = hip_pos.x - head_pos.x;
        
        println!("====================offset_x: {}",position.x);
        //ganon's offset is 8.321199
        x_offset = (8.321199 - position.x)/3.0;



        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_DX
            );
            x_offset = 9.0 - height;
            y_offset = -1.0;
            z_offset = -1.0;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_GIRL
            );
            x_offset = 17.0 - height;
            y_offset = 0.5;
            z_offset = -1.0;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_BIG
            );
            y_offset = 0.0;
            z_offset = 0.0;
            x_offset = 19.0 - height;
        }
        if fighter_kind == *FIGHTER_KIND_YOSHI {
            x_offset = -4.0;
        }
        if fighter_kind == *FIGHTER_KIND_PACKUN {
            x_offset = -7.0;
        }
        if fighter_kind == *FIGHTER_KIND_ROBOT {
            x_offset = -6.0;
        }
        if  [*FIGHTER_KIND_MIIFIGHTER,
            *FIGHTER_KIND_MIIGUNNER,
            *FIGHTER_KIND_MIISWORDSMAN,
            *FIGHTER_KIND_MIIENEMYF,
            *FIGHTER_KIND_MIIENEMYG,
            *FIGHTER_KIND_MIIENEMYS].contains(&fighter_kind) {
            x_offset = -2.0;
            z_offset = -2.5;
        }
        fighter.status_Thrown();
        let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
        LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_OFFSET_TRANSLATE | constraint_flags as i32) as u32);
        //WorkModule::set_int64(fighter.module_accessor, motion as i64,*FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
        LinkModule::set_constraint_translate_offset(fighter.module_accessor, &Vector3f{ x: x_offset, y: y_offset, z: z_offset });
        //CaptureModule::set_capture_node_offset(fighter.module_accessor, &Vector3f{ x: 10.0, y: 0.0, z: 0.0 });
        if !(is_flat_stage()){
            if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_GAMEWATCH {
                let z_scale = ModelModule::scale_z(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor,z_scale,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_GAMEWATCH_Z_SCALE);
                ModelModule::set_scale_z(fighter.module_accessor,1.0);
            }
        }

        //let pre_flip_state = MotionModule::is_flip(capture_boma);
        //WorkModule::set_flag(capture_boma,fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FLIP);
        //
        //if LinkModule::get_parent_lr(fighter.module_accessor,*LINK_NO_CAPTURE) < 0.0{
        //    println!("in get lr");
        //    MotionModule::set_flip(fighter.module_accessor,true,true,false);
        //    println!("flip has been set");
        //}
        println!("it happens");
        return 0.into();
    }
    println!("it goes past");
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}


pub unsafe extern "C" fn thrown_status_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);

    if !(is_flat_stage()){
        if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_GAMEWATCH {
            let z_scale = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_GAMEWATCH_Z_SCALE);
            ModelModule::set_scale_z(fighter.module_accessor,z_scale);
        }
    }
    if (StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER)
    && (utility::get_kind(&mut *parent_boma) == *FIGHTER_KIND_KAMUI) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}








pub unsafe extern "C" fn game_catchedpotspecialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        MotionModule::set_flip(agent.module_accessor,true,true,false);
        println!("hiiiiiiiiiiiiiiiiii");
    }
}



pub unsafe extern "C" fn caught_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
    if (StatusModule::status_kind(parent_boma) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(parent_boma) == FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP)
    && (utility::get_kind(&mut *parent_boma) == *FIGHTER_KIND_KAMUI) {
        let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
        let mut motion = hash40("catched_pot_final_scene_01");
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
        let egg_size = WorkModule::get_param_float(fighter.module_accessor, hash40("egg_collision_size"), 0);
        let mut x_offset = 22.0 - height;
        let mut z_offset = (7.2 -egg_size)/4.0;
        let mut y_offset = 0.0;

        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

        let head_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let position = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), hip_pos, true);
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("head"), head_pos, true);
        position.x = hip_pos.x - head_pos.x;
        
        println!("====================offset_x: {}",position.x);
        //ganon's offset is 8.321199
        x_offset = (8.321199 - position.x)/3.0;



        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_DX
            );
            x_offset = 9.0 - height;
            y_offset = -1.0;
            z_offset = -1.0;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_GIRL
            );
            x_offset = 17.0 - height;
            y_offset = 0.5;
            z_offset = -1.0;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_BIG
            );
            y_offset = 0.0;
            z_offset = 0.0;
            x_offset = 19.0- height;
        }
        if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_YOSHI {
            x_offset = -4.0;
        }
        if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_PACKUN {
            x_offset = -7.0;
        }
        if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_ROBOT {
            x_offset = -6.0;
        }
        if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIIFIGHTER
        || utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIIGUNNER
        || utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIISWORDSMAN
        || utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIIENEMYF
        || utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIIENEMYG
        || utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_MIIENEMYS {
            x_offset = -2.0;
            z_offset = -2.5;
        }

        let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
        LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_OFFSET_TRANSLATE | constraint_flags as i32) as u32);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
        LinkModule::set_constraint_translate_offset(fighter.module_accessor, &Vector3f{ x: x_offset, y: y_offset, z: z_offset });
        if !(is_flat_stage()){
            if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_GAMEWATCH {
                let z_scale = ModelModule::scale_z(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor,z_scale,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_GAMEWATCH_Z_SCALE);
                ModelModule::set_scale_z(fighter.module_accessor,1.0);
            }
        }
    }
    println!("after the if");
    println!("right before shift status");
    fighter.sub_shift_status_main(L2CValue::Ptr(caught_final_main_loop as *const () as _))
}



pub unsafe extern "C" fn caught_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("before on flag");
    println!("before 0");
    return 0.into();
}


pub unsafe extern "C" fn caught_final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if utility::get_kind(&mut (*fighter.module_accessor)) == *FIGHTER_KIND_GAMEWATCH {
        let z_scale = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_GAMEWATCH_Z_SCALE);
        ModelModule::set_scale_z(fighter.module_accessor,z_scale);
    }
    return L2CValue::I32(0)
}

