use super::*;





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




unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2);

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

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut frame = 0.0;
    if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED){
        frame = 10.0;
        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
        WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2);
    }
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
        WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2);
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



unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON) {
    //    ArticleModule::remove_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,ArticleOperationTarget(0));
    //}
    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    ShieldModule::clear_all(module_accessor);

    //smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    return L2CValue::I32(0)

}


unsafe extern "C" fn special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON) {
    //    ArticleModule::remove_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,ArticleOperationTarget(0));
    //}
    ShieldModule::clear_all(module_accessor);

    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    //smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD)(fighter)
    return L2CValue::I32(0)

}


unsafe extern "C" fn special_n_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //if ArticleModule::is_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON) {
    //    ArticleModule::remove_exist(module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,ArticleOperationTarget(0));
    //}
    ShieldModule::clear_all(module_accessor);

    ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    //ShieldModule::clear_all_2
    //WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
    //smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD)(fighter)
    return L2CValue::I32(0)

}




pub fn install(agent: &mut Agent) {
    agent.status(Main , *FIGHTER_STATUS_KIND_SPECIAL_N,                 special_n_main);
    agent.status(End  , *FIGHTER_STATUS_KIND_SPECIAL_N,                 special_n_end);

    agent.status(Main , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,      special_n_hold_main);
    agent.status(End  , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,      special_n_hold_end);

    agent.status(Main , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,     special_n_shoot_main);
    agent.status(End  , *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,     special_n_shoot_end);
}