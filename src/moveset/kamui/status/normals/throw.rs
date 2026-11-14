use super::*;

unsafe extern "C" fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_TRISHULA_EFFECT_ID);
    if (effect_id != 0) && EffectModule::is_exist_effect(fighter.module_accessor,effect_id as u32){
        EffectModule::kill(fighter.module_accessor, effect_id as u32, true, true);
    }
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_TRISHULA_EFFECT_ID);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter);
}


pub fn install(agent: &mut Agent) {
    agent.status(End,   *FIGHTER_STATUS_KIND_THROW,        throw_end);
}