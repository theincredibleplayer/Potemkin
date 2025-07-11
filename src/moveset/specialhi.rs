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


pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon,anim_name: &str, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let capture_boma = sv_battle_object::module_accessor(capture_id as u32);

    if capture_id != 0x50000000 {
        //println!("playing anim");
        //MotionModule::change_motion(captured_boma,Hash40::new("capture_wait_hi"),0.0,1.0,false,0.0,false,false);

        // finds out the body type of the opponent and changes the motion variable to that
        let motion_share = WorkModule::get_param_int(capture_boma,0xcad2ee25e,0xc07d88ea0); //grabs the body type param
        let mut motion = hash40(anim_name);
        let mut share_type = 0;
        
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(
                capture_boma,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_DX
            );
        }

        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(
                capture_boma,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_GIRL
            );
        }

        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(
                capture_boma,
                Hash40::new_raw(motion),
                *BODY_TYPE_MOTION_BIG
            );
        }

        //let pre_flip_state = MotionModule::is_flip(capture_boma);
        //WorkModule::set_flag(capture_boma,boi,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FLIP);
        // this should flip the opponent if im facing left but it doesnt work for some reason??
        //if PostureModule::lr(fighter.module_accessor) < 0.0 {
        //    //println!("flipped allegedly from elsewhere");
        //    PostureModule::set_lr(capture_boma, 1.0);
        //    PostureModule::update_rot_y_lr(capture_boma);
        //    MotionModule::set_flip(capture_boma,true,true,false);
        //} else {
        //    PostureModule::set_lr(capture_boma, -1.0);
        //    PostureModule::update_rot_y_lr(capture_boma);
        //    MotionModule::set_flip(capture_boma,false,true,false);
        //}

        //plays the anim
        MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), 0.0, mot_rate, false, 0.0, false, false);
        //println!("flipped state: {}", boi);
    }
}

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD);
        return 1.into()
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(module_accessor,Hash40::new("special_hi"),0.0,1.0,false,0.0,false,false);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_hi"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi"),-1.0,1.0,0.0,false,false);
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



pub unsafe extern "C" fn special_hi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let scale = PostureModule::scale(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);            
    macros::CAM_ZOOM_IN_arg6(
        fighter,
        /*frames to zoom in totally*/10.0,
        /*according to calude the object id?*/0.0,
        /*the amt of zoom needed, higher the number lower the zoom*/1.1*scale,
        /*y_rot*/0.1, // up and down
        /*x_rot*/0.2 * lr, //left and right
        true,
    );
    fighter.clear_lua_stack();
    lua_args!(fighter,
    *MA_MSC_CMD_CAMERA_CAM_OFFSET,
    /*x*/12.0 * lr,
    /*y*/14.0,
    /*z*/0);
    smash::app::sv_module_access::camera(fighter.lua_state_agent);


    ArticleModule::generate_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_l"),false,-1.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_start"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_start"),0.0,1.0,false,0.0,false,false);
    }

    WorkModule::set_int(module_accessor,5,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI); //sets the amount of times the loop anim loops

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_start_main_loop as *const () as _))
}

unsafe fn special_hi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    grabbed_anim_selector(fighter,"catched_pot_special_hi",1.0); //plays the caught anim on the opponent on every frame since i want it to lock to a specific frame (i will implement a custom anim later)

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_hi_start"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi_start"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_hi_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    fighter.status_end_Throw()
}

pub unsafe extern "C" fn special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let scale = PostureModule::scale(fighter.module_accessor);
    macros::CAM_ZOOM_OUT(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);            
    macros::CAM_ZOOM_IN_arg6(
        fighter,
        /*frames to zoom in totally*/90.0,
        /*according to calude the object id?*/0.0,
        /*the amt of zoom needed, higher the number lower the zoom*/1.1*scale,
        /*y_rot*/0.1, // up and down
        /*x_rot*/-0.3 * lr, //left and right
        /*this seems to be a quake multiplier*/ true,
    );
    fighter.clear_lua_stack();
    lua_args!(fighter,
    *MA_MSC_CMD_CAMERA_CAM_OFFSET,
    /*x*/12.0 * lr,
    /*y*/17.0,
    /*z*/0.0);
    smash::app::sv_module_access::camera(fighter.lua_state_agent);

    ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_r"),false,-1.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_loop"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_loop"),0.0,1.0,false,0.0,false,false);
    }

    grabbed_anim_selector(fighter,"catched_pot_special_hi",1.0); //same as the other one but this time it plays it on the first hit

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_loop_main_loop as *const () as _))
}

unsafe fn special_hi_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_hi_loop"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi_loop"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    // code to loop special_hi loop the amount of times set by the FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI int (it was set in special_hi_start_main)
    if MotionModule::is_end(fighter.module_accessor) {

        if WorkModule::get_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI) <= 0 {
            let scale = PostureModule::scale(fighter.module_accessor);
            macros::CAM_ZOOM_OUT(fighter);
            let lr = PostureModule::lr(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);            
            macros::CAM_ZOOM_IN_arg6(
                fighter,
                /*frames to zoom in totally*/50.0,
                /*according to calude the object id?*/0.0,
                /*the amt of zoom needed, higher the number lower the zoom*/2.0*scale,
                /*y_rot*/0.13, // up and down
                /*x_rot*/-0.7 * lr, //left and right
                /*this seems to be a quake multiplier*/ true,
            );
            fighter.clear_lua_stack();
            lua_args!(fighter,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/12.0 * lr,
            /*y*/17.0,
            /*z*/0.0);
            smash::app::sv_module_access::camera(fighter.lua_state_agent);
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE.into(),false.into());
        } else {
            if WorkModule::get_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI) == 1 {

            }
            WorkModule::dec_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI);

            ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_r"),false,-1.0);

            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_loop"),0.0,1.0,false,0.0,false,false);
            }
            else {
                GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::change_motion(module_accessor,Hash40::new("special_hi_loop"),0.0,1.0,false,0.0,false,false);
                grabbed_anim_selector(fighter,"catched_pot_special_hi",1.0); //same as the other ones but this plays it on every hit
            }
        }
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_hi_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    fighter.status_end_Throw()
}


pub unsafe extern "C" fn special_hi_release_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let lr = PostureModule::lr(fighter.module_accessor);

    fighter.clear_lua_stack();
    lua_args!(fighter,
    *MA_MSC_CMD_CAMERA_CAM_OFFSET,
    /*x*/12.0 * lr,
    /*y*/17.0,
    /*z*/0);
    smash::app::sv_module_access::camera(fighter.lua_state_agent);



    ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("win_3"),false,0.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_release"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_release"),0.0,1.0,false,0.0,false,false);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_release_main_loop as *const () as _))
}

unsafe fn special_hi_release_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);


    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY){
        grabbed_anim_selector(fighter,"catched_pot_special_hi",0.1); // same as the above but plays on the final hit

        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY);
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
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_hi_release"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    
    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi_release"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }


    // also depricated code from the faked grab
    /*if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RELEASE_ENEMY) {
        let enemy_id = WorkModule::get_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID) as u32;
        let enemy = sv_battle_object::module_accessor(enemy_id);
        LinkModule::remove_model_constraint(enemy,true);
        if LinkModule::is_link(enemy,*LINK_NO_CAPTURE) {
            LinkModule::unlink(enemy,*LINK_NO_CAPTURE);
        }
        let pos = Vector2f{x: PostureModule::pos_x(module_accessor) + (20.0 * PostureModule::lr(module_accessor)), y: PostureModule::pos_y(module_accessor) + 30.0};
        PostureModule::set_pos_2d(enemy,&pos);
        WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RELEASE_ENEMY);
    }*/


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

pub unsafe extern "C" fn special_hi_release_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
    CameraModule::zoom_out(module_accessor, 0);
    fighter.status_end_Throw()
}

pub unsafe extern "C" fn special_hi_release_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //handles releasing since the normal acmd ints arent set properly (i should probably research that later)
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
            }
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE);
        }
    }
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP
    && StatusModule::status_kind_next(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START
    && StatusModule::status_kind_next(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE{
        CameraModule::zoom_out(fighter.module_accessor, 0);
        fighter.sub_throw_uniq_process_exit();
    }
    return L2CValue::I32(0)
}