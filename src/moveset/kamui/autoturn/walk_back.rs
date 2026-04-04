use super::*;





unsafe extern "C" fn sub_walk_uniq_check(fighter: &mut L2CFighterCommon) {
    fighter.sub_walk_uniq_check();
}




unsafe extern "C" fn walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.status_Walk_Common();
    if StopModule::is_stop(fighter.module_accessor) == false {
        fighter.sub_walk_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(sub_walk_uniq_check as *const () as _));   
    fighter.sub_shift_status_main(L2CValue::Ptr(walk_main_loop as *const () as _))
}



unsafe extern "C" fn walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.status_Walk_Main_common((*FIGHTER_STATUS_KIND_WALK_BRAKE).into(), (fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor)).into(), true.into(), L2CValue::Ptr(walk_function_idfk as *const () as _));

    return 0.into();
}


unsafe extern "C" fn walk_function_idfk(fighter: &mut L2CFighterCommon) -> bool{
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != 0.0 )
    && (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != PostureModule::lr(module_accessor)) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
        return true.into()
    }
    return false.into()
}




unsafe extern "C" fn walkback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk_Common();
    if StopModule::is_stop(fighter.module_accessor) == false {
        fighter.sub_walk_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(sub_walk_uniq_check as *const () as _));   
    fighter.sub_shift_status_main(L2CValue::Ptr(walkback_main_loop as *const () as _))
}

unsafe extern "C" fn walkback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.status_Walk_Main_common((*FIGHTER_STATUS_KIND_WALK_BRAKE).into(), (fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor)).into(), true.into(), L2CValue::Ptr(walk_function_idfk as *const () as _));
    fighter.status_Walk_Main_common(
        (FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK).into(),
        (-PostureModule::lr(fighter.module_accessor) * fighter.global_table[STICK_X].get_f32()).into(),
        false.into(),
        L2CValue::Ptr(walk_function_idfk as *const () as _)
    );
    return 0.into();
}

unsafe extern "C" fn walkback_calc_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.FighterStatusUniqProcessWalkBack_calc_param();
    return 0.into();
}

unsafe extern "C" fn walkback_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_exec();
    return 0.into();
}
unsafe extern "C" fn walkback_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_init();
    return 0.into();
}
unsafe extern "C" fn walkback_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_exit();
    return 0.into();
}
unsafe extern "C" fn walkback_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_WalkBack();
    return 0.into();
}
unsafe extern "C" fn walkback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_WalkBack();
    return 0.into();
}

unsafe extern "C" fn walk_back_brake_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_brake_uniq_process_exec();
    return 0.into();
}
unsafe extern "C" fn walk_back_brake_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_brake_uniq_process_init();
    return 0.into();
}
unsafe extern "C" fn walk_back_brake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_WalkBrake();
    return 0.into();
}
unsafe extern "C" fn walk_back_brake_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_WalkBrakeBack();
    return 0.into();
}
unsafe extern "C" fn walk_back_brake_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_WalkBrake();
    return 0.into();
}


pub fn install(agent: &mut Agent) {

    agent.status(Pre       , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_pre);
    agent.status(Main      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_main);
    agent.status(Exit      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_exit);
    agent.status(Init      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_init);
    agent.status(Exec      , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_exec);
    agent.status(End       , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_end);
    agent.status(CalcParam , FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,               walkback_calc_param);

    agent.status(Pre       , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK,         walk_back_brake_pre);
    agent.status(Main      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK,         walk_back_brake_main);
    agent.status(Init      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK,         walk_back_brake_init);
    agent.status(Exec      , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK,         walk_back_brake_exec);
    agent.status(End       , FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK,         walk_back_brake_end);


}