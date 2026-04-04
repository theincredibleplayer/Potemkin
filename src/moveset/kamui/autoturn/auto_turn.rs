use super::*;



unsafe extern "C" fn autoturn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait();
    return 0.into();
}

unsafe extern "C" fn autoturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    PostureModule::reverse_lr(fighter.module_accessor);
    if ControlModule::is_remake_command(fighter.module_accessor) {
        fighter.sub_fighter_remake_control_command();
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(autoturn_main_loop as *const () as _))
}

unsafe extern "C" fn autoturn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_wait_common_Main().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return 0.into();
}

unsafe extern "C" fn autoturn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Wait();
    return 0.into();
}









pub fn install(agent: &mut Agent) {
    agent.status(Pre,   FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN,    autoturn_pre);
    agent.status(Main,  FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN,    autoturn_main);
    agent.status(End,   FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN,    autoturn_end);
}