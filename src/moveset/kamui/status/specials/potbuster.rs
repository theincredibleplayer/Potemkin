use super::*;


unsafe extern "C" fn potbusterwhiff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(module_accessor,Hash40::new("potemkin_buster_whiff"),0.0,1.0,false,0.0,false,false);
    
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
    let mut lr = PostureModule::lr(fighter.module_accessor);
    fighter.push_lua_stack(&mut L2CValue::F32(lr));
    sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(potbusterwhiff_main_loop as *const () as _))
}

unsafe fn potbusterwhiff_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
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


unsafe extern "C" fn potbuster_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(module_accessor,Hash40::new("potemkin_buster"),0.0,1.0,false,0.0,false,false);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(potbuster_main_loop as *const () as _))
}

unsafe fn potbuster_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw"), Hash40::new("invalid"), true, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
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

unsafe extern "C" fn potbuster_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CameraModule::zoom_out(fighter.module_accessor, 0);
    lua_bind::FighterCutInManager::request_end(singletons::FighterCutInManager());
    if CatchModule::is_catch(fighter.module_accessor) {
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor,false,false);
        println!("catch cut was ran");
    }
    return L2CValue::I32(0)
}



pub fn install(agent: &mut Agent) {
    agent.status(Main, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF, potbusterwhiff_main);

    agent.status(Main, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, potbuster_main);
    agent.status(End, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, potbuster_end);
}