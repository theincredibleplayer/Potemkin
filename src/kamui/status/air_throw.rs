use super::*;

unsafe extern "C" fn airgrab_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_CATCH_ATTACK as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_CATCH as u32, 0);
    0.into()
}

unsafe extern "C" fn airgrab_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_air"), 0.0, 1.0, false, 0.0, false, false);
    grabbed_anim_selector(fighter, "catched_pot_special_hi", 0.1);
    fighter.sub_shift_status_main(L2CValue::Ptr(airgrab_main_loop as *const () as _))
}

unsafe fn airgrab_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    0.into()
}

unsafe extern "C" fn airgrab_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if CatchModule::is_catch(boma) {
        CatchModule::set_send_cut_event(boma, true);
        CatchModule::catch_cut(boma, false, false);
    }    
    0.into()
}

pub fn install() {
    Agent::new("kamui")
    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, airgrab_end)
    .install();
}