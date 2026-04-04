use super::*;



unsafe extern "C" fn fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.sub_fall_uniq_process_init();
    if FIGHTER_KAMUI_STATUS_KIND_WALK_BACK == fighter.global_table[PREV_STATUS_KIND] {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_FALL_FLAG_FROM_GROUND);
    }
    return 0.into();
}



pub fn install(agent: &mut Agent) {
    agent.status(Init,  *FIGHTER_STATUS_KIND_FALL,          fall_init);
    agent.status(Init,  *FIGHTER_STATUS_KIND_FALL_AERIAL,   fall_init);
    agent.status(Init,  *FIGHTER_STATUS_KIND_FALL_SPECIAL,  fall_init);
}