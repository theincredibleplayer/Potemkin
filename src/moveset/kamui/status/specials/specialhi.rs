use super::*;



unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD);
        return 1.into()
    }
    StatusModule::init_settings
    (fighter.module_accessor,
    SituationKind(*SITUATION_KIND_NONE),
    *FIGHTER_KINETIC_TYPE_NONE,
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
    (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
    (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
    (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
    0);
return L2CValue::I32(0)
    //return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
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
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
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



unsafe extern "C" fn special_hi_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0);
    return L2CValue::I32(0)
}

unsafe extern "C" fn special_hi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let scale = PostureModule::scale(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    JostleModule::set_status(fighter.module_accessor, false);
    if camera_checks(fighter){
        CameraModule::reset_all(fighter.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            fighter,
            /*frames to zoom in totally*/10.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/1.1*scale*1.3,
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
    }

    ArticleModule::generate_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    potemkin_mirror_handler(module_accessor);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_l"),false,-1.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
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
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP.into(),false.into());
    }
    return L2CValue::I32(0)
}



unsafe extern "C" fn special_hi_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0);
    return L2CValue::I32(0)
}

unsafe extern "C" fn special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let scale = PostureModule::scale(fighter.module_accessor);
    JostleModule::set_status(fighter.module_accessor, false);
    if camera_checks(fighter){
        macros::CAM_ZOOM_OUT(fighter);
        let lr = PostureModule::lr(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);            
        macros::CAM_ZOOM_IN_arg6(
            fighter,
            /*frames to zoom in totally*/90.0,
            /*according to calude the object id?*/0.0,
            /*the amt of zoom needed, higher the number lower the zoom*/1.1*scale*1.3,
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
    }
    ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    potemkin_mirror_handler(module_accessor);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_r"),false,-1.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
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
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    // code to loop special_hi loop the amount of times set by the FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI int (it was set in special_hi_start_main)
    if MotionModule::is_end(fighter.module_accessor) {

        if WorkModule::get_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI) <= 0 {
            if camera_checks(fighter){
                let scale = PostureModule::scale(fighter.module_accessor);
                macros::CAM_ZOOM_OUT(fighter);
                let lr = PostureModule::lr(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);            
                macros::CAM_ZOOM_IN_arg6(
                    fighter,
                    /*frames to zoom in totally*/50.0,
                    /*according to calude the object id?*/0.0,
                    /*the amt of zoom needed, higher the number lower the zoom*/2.0*scale*1.3,
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
            }
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE.into(),false.into());
        } else {
            if WorkModule::get_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI) == 1 {

            }
            WorkModule::dec_int(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI);

            ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("entry_r"),false,-1.0);

            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            }
            else {
                GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                MotionModule::change_motion(module_accessor,Hash40::new("special_hi_loop"),0.0,1.0,false,0.0,false,false);
                grabbed_anim_selector(fighter,"catched_pot_special_hi",1.0); //same as the other ones but this plays it on every hit
            }
        }
    }
    return L2CValue::I32(0)
}

unsafe extern "C" fn special_hi_release_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
    (fighter.module_accessor,
    SituationKind(*SITUATION_KIND_NONE),
    *FIGHTER_KINETIC_TYPE_NONE,
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
    (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
    (0) as u32,
    (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
    0);
    return L2CValue::I32(0)
}

unsafe extern "C" fn special_hi_release_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let lr = PostureModule::lr(fighter.module_accessor);
    JostleModule::set_status(fighter.module_accessor, false);
    fighter.clear_lua_stack();
    lua_args!(fighter,
    *MA_MSC_CMD_CAMERA_CAM_OFFSET,
    /*x*/12.0 * lr,
    /*y*/17.0,
    /*z*/0);
    smash::app::sv_module_access::camera(fighter.lua_state_agent);



    ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
    potemkin_mirror_handler(module_accessor);
    ArticleModule::change_motion(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("win_3"),false,0.0);

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
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
    let frame = MotionModule::frame(module_accessor);


    ArticleModule::set_frame(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, frame);


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
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
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

unsafe extern "C" fn special_hi_release_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //handles releasing since the normal acmd ints arent set properly (i should probably research that later)
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                JostleModule::set_status(fighter.module_accessor, true);
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
            }
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE);
        }
    }
    return L2CValue::I32(0)
}


unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
    if StatusModule::status_kind_next(module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP
    && StatusModule::status_kind_next(module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START
    && StatusModule::status_kind_next(module_accessor) != FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE{
        ArticleModule::remove_exist(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,ArticleOperationTarget(0));
        CameraModule::reset_main_camera_fov(module_accessor);
        CameraModule::zoom_out(module_accessor, 0);
        if CatchModule::is_catch(module_accessor) {
            CatchModule::set_send_cut_event(module_accessor, true);
            CatchModule::catch_cut(module_accessor,false,false);
            println!("catch cut was ran");
        }    
    }
    return L2CValue::I32(0)
}



pub fn install(agent: &mut Agent) {
    agent.status(Pre,   *FIGHTER_STATUS_KIND_SPECIAL_HI,                special_hi_pre);
    agent.status(Main,  *FIGHTER_STATUS_KIND_SPECIAL_HI,                special_hi_main);
    agent.status(End,   *FIGHTER_STATUS_KIND_SPECIAL_HI,                special_hi_end);

    agent.status(Pre,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START,     special_hi_start_pre);
    agent.status(Main,  FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START,     special_hi_start_main);
    agent.status(End,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START,     special_hi_end);

    agent.status(Pre,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP,      special_hi_loop_pre);
    agent.status(Main,  FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP,      special_hi_loop_main);
    agent.status(End,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP,      special_hi_end);

    agent.status(Pre,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE,   special_hi_release_pre);
    agent.status(Main,  FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE,   special_hi_release_main);
    agent.status(End,   FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE,   special_hi_end);
    agent.status(Exec,  FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE,   special_hi_release_exec);
}