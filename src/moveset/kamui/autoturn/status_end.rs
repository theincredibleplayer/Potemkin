use super::*;


pub unsafe extern "C" fn should_not_reverse_lr(status_kind: i32) -> bool {
    // add here to avoid it suddenly turning around at the end of the status
    if [
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        //*FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_GUARD_OFF,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_DASH,
        *FIGHTER_STATUS_KIND_CATCH_TURN,
        *FIGHTER_STATUS_KIND_CATCH_CUT,
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_SLIP_STAND,
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_SLIP_STAND_F,
        *FIGHTER_STATUS_KIND_SLIP_STAND_B,
        *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S3,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
        *FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
        *FIGHTER_STATUS_KIND_APPEAL,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING
    ].contains(&status_kind){
        true
    }else{
        false
    }
}

pub unsafe extern "C" fn auto_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    let prev_status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    unsafe fn update_lr(fighter: &mut L2CFighterCommon, lr: f32) {
        PostureModule::set_lr(fighter.module_accessor, lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);

    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr == 0.0
    || PostureModule::lr(fighter.module_accessor) == lr
    || StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        return false.into();
    }



    // add here to make it force turn around once the attack starts
    if ![
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
    ].contains(&next_status) {
        return false.into();
    }
    if [*FIGHTER_STATUS_KIND_FINAL].contains(&next_status){
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START){
            return false.into();
        }
    }
    if next_status == *FIGHTER_STATUS_KIND_WAIT 
    && should_not_reverse_lr(prev_status) {
        return false.into();
    }

    // item throw
    let cat3 = fighter.global_table[CMD_CAT3].get_i32();
    if next_status == *FIGHTER_STATUS_KIND_ITEM_THROW
    && situation_kind == *SITUATION_KIND_GROUND
    && !(cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 == 0)
    && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI == 0
    && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW == 0 {
        return false.into();
    }

    update_lr(fighter, lr);
    return true.into();
}
