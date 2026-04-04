use super::*;





unsafe extern "C" fn turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != 0.0 )
    && (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == PostureModule::lr(module_accessor)) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_WALK_BACK);
        return 1.into();
    }
    fighter.status_pre_Turn();
    return 0.into();
}




pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_TURN, turn_pre);
}