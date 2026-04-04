use super::*;



unsafe extern "C" fn air_autoturn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Fall();
    return 0.into();
}

unsafe extern "C" fn air_autoturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Fall();
    PostureModule::reverse_lr(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(air_autoturn_main_loop as *const () as _))
}

unsafe extern "C" fn air_autoturn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Fall_Main();
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FALL{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }else if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FALL_AERIAL{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(),false.into());
        }else{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    
    return 0.into();
}

unsafe extern "C" fn air_autoturn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Fall();
    return 0.into();
}



pub fn install(agent: &mut Agent) {
    agent.status(Pre,       FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN,    air_autoturn_pre);
    agent.status(Main,      FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN,    air_autoturn_main);
    agent.status(End,       FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN,    air_autoturn_end);
}