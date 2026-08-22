use super::*;



unsafe extern "C" fn rebound_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::enable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);

    let rate = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
    MotionModule::change_motion(module_accessor, Hash40::new("capture_jump"), 0.0, rate, false, 0.0, false, false);
    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.clear_lua_stack();
    lua_args!(fighter,*MA_MSC_CMD_PHYSICS_SET_SWING_INTP,false);
    sv_module_access::physics(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    fighter.sub_shift_status_main(L2CValue::Ptr(rebound_jump_main_loop as *const () as _))
}


unsafe extern "C" fn rebound_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    //CancelModule::enable_cancel(fighter.module_accessor);
    let rate = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        println!("switch to grounded");
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("capture_cut"),-1.0,rate,0.0,false,false);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_landingheavy"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_landingheavy"), -1);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            println!("transition to wait");
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        } else {
            println!("transition to fall");
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.sub_wait_ground_check_common(false.into());
        } else {
            fighter.sub_air_check_fall_common();
        }
        return 0.into();
    }
    return 0.into();
}

unsafe extern "C" fn rebound_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
        StatusModule::init_settings
            (fighter.module_accessor,
            SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_REBOUND_JUMP,
            *GROUND_CORRECT_KIND_AIR as u32,
            GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_REBOUND_JUMP_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_REBOUND_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_REBOUND_FLOAT,
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



unsafe extern "C" fn rebound_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End   , fighter, *FIGHTER_STATUS_KIND_REBOUND_JUMP)(fighter)
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_main);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, rebound_jump_end);

}