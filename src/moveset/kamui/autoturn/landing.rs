use super::*;



pub unsafe extern "C" fn landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != 0.0 ){
        if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != PostureModule::lr(module_accessor)) {
            WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_LANDING_FLAG_PROG_TURN_RESERVE);
        }else{
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
        }
    }
    fighter.sub_landing_uniq_process_init();
    return 0.into();
}




pub fn install(agent: &mut Agent) {
    agent.status(Init,  *FIGHTER_STATUS_KIND_LANDING,               landing_init);
    agent.status(Init,  *FIGHTER_STATUS_KIND_LANDING_LIGHT,         landing_init);
    agent.status(Init,  *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,  landing_init);
}