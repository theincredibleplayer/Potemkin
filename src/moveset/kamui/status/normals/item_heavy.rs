use super::*;


unsafe extern "C" fn item_heavy_wait_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("item_heavy_wait") {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("item_heavy_wait"),0.0,1.0,false,0.0,false,false);
    }
    return 0.into();
    // smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ITEM_LIFT_WAIT)(fighter);
}



pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ITEM_LIFT_WAIT, item_heavy_wait_exec);
}