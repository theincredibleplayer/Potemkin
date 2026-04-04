use super::*;

unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
    //GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if situation_change == false {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s"),0.0,1.0,false,0.0,false,false);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_s"),-1.0,1.0,0.0,false,false);
    }
}

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if ( stick_x * PostureModule::lr(fighter.module_accessor) ) > 0.0{
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F);
        }else{
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B);
        }
        return 1.into()
    }
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    motion_setup(fighter,false);
    let module_accessor = fighter.module_accessor;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
    //KineticModule::add_speed(fighter.module_accessor,&smash::phx::Vector3f { x: 15.0, y: 0.0, z: 0.0 });
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut current_speed = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
    let lua_state = fighter.lua_state_agent;
    let lr = PostureModule::lr(fighter.module_accessor);
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(fighter.module_accessor,0.5 + current_speed,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL );
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, current_speed * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL.into(),false.into());
    }
    if (ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false
    && ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_ATTACK_RAW) == false)
    || GroundModule::is_ottotto_lr(fighter.module_accessor, lr, 1.5) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END.into(),false.into());
        return L2CValue::I32(0)
    }
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_slide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_slide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_slide"),0.0,1.0,false,0.0,false,false);
    JostleModule::set_status(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_slide_loop as *const () as _))
}

unsafe fn side_b_slide_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let lr = PostureModule::lr(fighter.module_accessor);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, 3.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let limiting_speed = 5.0;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return L2CValue::I32(0)
    }
    if (ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false
    && ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_ATTACK_RAW) == false)
    || GroundModule::is_ottotto_lr(fighter.module_accessor, lr, 1.5) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END.into(),false.into());
        return L2CValue::I32(0)
    }
    /*if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
          fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK.into(),false.into());
          return L2CValue::I32(0)
    }*/
    return L2CValue::I32(0)
}

unsafe extern "C" fn side_b_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_status(fighter.module_accessor, true);
    return L2CValue::I32(0)
}

unsafe extern "C" fn side_b_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    //KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_end"),0.0,1.0,false,0.0,false,false);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.7,0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_end_loop as *const () as _))
}

unsafe fn side_b_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    //KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_attack"),0.0,1.0,false,0.0,false,false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(side_b_attack_loop as *const () as _))
}

unsafe fn side_b_attack_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_slide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_slide_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}



unsafe extern "C" fn side_b_slide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}


unsafe extern "C" fn megafist_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
        (fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        (0) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0);
    return L2CValue::I32(0)
}


unsafe extern "C" fn megafist_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;

    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);

    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_mf_f"),0.0,1.0,false,0.0,false,false);
    } else {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_mf_f"),0.0,1.0,false,0.0,false,false);
    }

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,3.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);


    


    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,3.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    let speed_const;
    if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F {
        speed_const = 0.8;
    } else {
        speed_const = -0.8;
    }
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, speed_const * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.2, -0.2);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    //KineticModule::add_speed(module_accessor, &Vector3f{x:30.0,y:30.0,z:0.0});
    fighter.sub_shift_status_main(L2CValue::Ptr(megafist_f_main_loop as *const () as _))
}

unsafe fn megafist_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_const;
    if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F {
        speed_const = 0.8;
    } else {
        speed_const = -0.8;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, speed_const * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    if StatusModule::is_situation_changed(fighter.module_accessor) == true  {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING.into(),false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}


unsafe extern "C" fn megafist_f_land_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    //MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_s_mf_f_land"),-1.0,1.0,0.0,false,false);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F{
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_mf_f_land"),0.0,1.0,false,0.0,false,false);
    } else {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s_mf_f_land"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(megafist_f_land_main_loop as *const () as _))
}


unsafe extern "C" fn megafist_f_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}


unsafe extern "C" fn megafist_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}



unsafe extern "C" fn megafist_b_land_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    KineticModule::clear_speed_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}




pub fn install(agent: &mut Agent) {
    agent.status(Pre,   *FIGHTER_STATUS_KIND_SPECIAL_S,                             special_s_pre);
    agent.status(Main,  *FIGHTER_STATUS_KIND_SPECIAL_S,                             special_s_main);
            
    agent.status(Pre,   *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide_pre);
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide);
            
    agent.status(Pre,   *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,              side_b_end_pre);
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,              side_b_end);
            
    agent.status(Pre,   *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,                side_b_attack_pre);
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,                side_b_attack);
            
    agent.status(Init,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide_init);
    agent.status(Exit,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide_exit);
    agent.status(Exec,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide_exec);
    agent.status(End,   *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,                  side_b_slide_end);

    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, megafist_f_pre)
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,         megafist_f_main);
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING, megafist_f_land_main);
    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, megafist_f_pre)
    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,         megafist_f_main);

}