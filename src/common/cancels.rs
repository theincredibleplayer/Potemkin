use super::*;

//Cancels
pub unsafe extern "C" fn cancels(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let status = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);
    let cat2 = ControlModule::get_command_flag_cat(boma, 1);
    let cat3 = ControlModule::get_command_flag_cat(boma, 2);
    let cat4 = ControlModule::get_command_flag_cat(boma, 3);
    let control = ControlModule::get_attack_air_kind(boma);
    let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let remaining_jumps= max_jump_count-jump_count;
    let aerials = [
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_N
    ];
    if CancelModule::is_enable_cancel(boma) || fighter.global_table[IS_STOP].get_bool()|| !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }
    if [
        *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, 
        *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0 && FighterManager::is_final(singletons::FighterManager()){
            WorkModule::off_flag(boma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(boma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(boma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::on_flag(boma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            WorkModule::on_flag(boma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
        }
        if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND) != 0 {
            StatusModule::change_status_request_from_script(boma, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF, false);
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0 && situation_kind == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(boma, FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD, false);
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, false);
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false); 
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
        if ![*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status) {
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
                if remaining_jumps > 0 {
                    if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
                        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);    
                    }
                    else{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, false);
                    }
                }
            }
        }
    }
    if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            for attack_air in aerials {
                if control == attack_air {
                    if !(motion_kind == hash40("attack_air_lw") && control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW) {
                        ControlModule::set_attack_air_kind(boma, attack_air);
                        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
        }
    }
    if status == *FIGHTER_STATUS_KIND_ATTACK {
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
        }
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status) {
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
        }
    }
}