use super::*;




pub unsafe extern "C" fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.status_pre_Wait_check_interrupt().get_bool() {
        return 1.into();
    }
    if !(WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == 0.0) {
        if !(WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == PostureModule::lr(fighter.module_accessor)) {
            let mut some_bool = false;
            let mut some_bool_2= false;
            some_function(&some_bool,fighter.global_table[PREV_STATUS_KIND].get_i32());
            if (some_bool == false) {
                if !(GroundModule::is_ottotto(fighter.module_accessor, 0.86)) {
                    some_bool_2 = false;
                } else {
                    if !(fighter.global_table[STICK_X].get_f32().abs() < 0.75) {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
                        return 1.into();
                    }
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO) {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
                        return 1.into();
                    }
                    some_bool_2 = true;
                }
                if some_bool_2 {
                    fighter.status_pre_Wait_main();
                    return 1.into();
                }
            }
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
            return 1.into();
        }
    }
    fighter.status_pre_Wait_main();
    return 1.into();
}


pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_KAMUI_STATUS_KIND_WALK_BACK{
        MotionModule::change_motion(module_accessor,Hash40::new("walk_stop_b"),0.0,1.0,false,0.0,false,false);
    }else if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_WALK{
        MotionModule::change_motion(module_accessor,Hash40::new("walk_stop_f"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let lr = PostureModule::lr(fighter.module_accessor);
    if lr_1on1 != 0.0 {
        if (lr_1on1 == lr)
        && (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK))
        && (WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) <= (PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(module_accessor)).abs() )
        && (WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y")) <= fighter.global_table[STICK_Y].get_f32())
        {
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_WALK_BACK.into(),true.into());
            return 0.into();
        }else if (lr_1on1 != lr){
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
            return 0.into();
        }
    }
    return 0.into();
}






pub fn install(agent: &mut Agent) {
    agent.status(Main,  *FIGHTER_STATUS_KIND_WAIT,      wait_main);
    agent.status(Pre,   *FIGHTER_STATUS_KIND_WAIT,      wait_pre);
}