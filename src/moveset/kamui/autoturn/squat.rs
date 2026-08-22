use super::*;

pub unsafe extern "C" fn squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_common(false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(squat_wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_SquatWait_Main().get_bool(){
        return 0.into();
    }
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr_1on1 != 0.0 && PostureModule::lr(fighter.module_accessor) != lr_1on1 {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_SQUAT_AUTO_TURN.into(), false.into());
    }
    return 0.into();
}



pub fn install(agent: &mut Agent) {
    agent.status(Main,  *FIGHTER_STATUS_KIND_SQUAT_WAIT,      squat_wait_main);
}