use super::*;





unsafe extern "C" fn attackhi3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()

}
unsafe extern "C" fn attacklw3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()

}
unsafe extern "C" fn attacks3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()

}
unsafe extern "C" fn attackhi4_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()

}
unsafe extern "C" fn attacklw4_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()

}
unsafe extern "C" fn attacks4_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_HI3,        attackhi3_init);
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_LW3,        attacklw3_init);
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_S3,         attacks3_init);
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_HI4_START,  attackhi4_init);
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_LW4_START,  attacklw4_init);
    agent.status(Init,   *FIGHTER_STATUS_KIND_ATTACK_S4_START,   attacks4_init);
}