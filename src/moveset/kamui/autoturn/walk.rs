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




pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WALK, walk_main);
}