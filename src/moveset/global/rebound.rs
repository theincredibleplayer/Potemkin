use super::*;



/*
unsafe extern "C" fn rebound_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::enable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(module_accessor, Hash40::new("capture_jump"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    fighter.clear_lua_stack();
    lua_args!(*MA_MSC_CMD_PHYSICS_SET_SWING_INTP,false);
    sv_module_access::physics(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    let cancel_frame = WorkModule::get_param_int(module_accessor, hash40("common"), hash40("catch_rebound_jump_cancel_frame"));
    WorkModule::set_int(module_accessor, cancel_frame, *FIGHTER_STATUS_REBOUND_JUMP_WORK_INT_CENCEL_FRAME);
    let is_stop = StopModule::is_stop(module_accessor);
    if is_stop {
        fighter.rebound_jump_uniq(false);
    }
    fighter.global_table[0x15].assign(&L2CValue::Ptr(fighter.bind_address_call_rebound_jump_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(caught_main_loop as *const () as _))
}


unsafe extern "C" fn rebound_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    return 0.into();
}
*/
unsafe extern "C" fn rebound_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
        StatusModule::init_settings
            (fighter.module_accessor,
            SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_REBOUND_JUMP,
            *GROUND_CORRECT_KIND_AIR as u32,
            GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_REBOUND_JUMP_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0);
        FighterStatusModuleImpl::set_fighter_status_data
            (fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_ENABLE,
            false,
            false,
            false,
            (0) as u64,
            (0) as u32,
            (0) as u32,
            0);
    return L2CValue::I32(0)
}


unsafe extern "C" fn rebound_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Main  , fighter, *FIGHTER_STATUS_KIND_REBOUND_JUMP)(fighter)
}

unsafe extern "C" fn rebound_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End   , fighter, *FIGHTER_STATUS_KIND_REBOUND_JUMP)(fighter)
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_main);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_end);

}