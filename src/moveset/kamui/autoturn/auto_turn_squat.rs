use super::*;



unsafe extern "C" fn squat_auto_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_SquatWait_common(FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE.into())
}

pub unsafe extern "C" fn squat_auto_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_common(true.into());
    //FighterUtil::set_turn_squat_auto(fighter.module_accessor);
    PostureModule::reverse_lr(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_turn"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(squat_auto_turn_main_loop as *const () as _))
}

pub unsafe extern "C" fn squat_auto_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if !fighter.status_SquatWait_Main().get_bool() && (MotionModule::motion_kind(fighter.module_accessor)!= hash40("squat_turn")){
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn squat_auto_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_SquatWait();
    0.into()
}



pub fn install(agent: &mut Agent) {
    agent.status(Main,  FIGHTER_KAMUI_STATUS_KIND_SQUAT_AUTO_TURN,      squat_auto_turn_main);
    agent.status(Pre,   FIGHTER_KAMUI_STATUS_KIND_SQUAT_AUTO_TURN,      squat_auto_turn_pre);
    agent.status(End,   FIGHTER_KAMUI_STATUS_KIND_SQUAT_AUTO_TURN,      squat_auto_turn_end);
}