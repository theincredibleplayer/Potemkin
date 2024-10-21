use crate::moveset::*;


unsafe extern "C" fn some_function(mut param_1:&bool, status_kind: i32) {
    if ![
    *FIGHTER_STATUS_KIND_DASH,
    *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
    *FIGHTER_STATUS_KIND_RUN_BRAKE,
    *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
    *FIGHTER_STATUS_KIND_SQUAT_RV,
    *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
    *FIGHTER_STATUS_KIND_GUARD_OFF,
    *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
    *FIGHTER_STATUS_KIND_ESCAPE,
    *FIGHTER_STATUS_KIND_ESCAPE_F,
    *FIGHTER_STATUS_KIND_ESCAPE_B,
    *FIGHTER_STATUS_KIND_ATTACK_DASH,
    *FIGHTER_STATUS_KIND_ATTACK,
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
    *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
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
    *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1
    ].contains(&status_kind){
        if  [
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B,
        *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP,
        *FIGHTER_RYU_STATUS_KIND_FINAL_LANDING,
        *FIGHTER_RYU_STATUS_KIND_FINAL2,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING
        ].contains(&status_kind){
            param_1 = &true;
            return ;
        } else {
            param_1 = &false;
            return ;
        }
    }
}
    

unsafe extern "C" fn update_lr(fighter: &mut L2CFighterCommon) {
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    PostureModule::set_lr(fighter.module_accessor, lr_1on1);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
}



pub unsafe extern "C" fn auto_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut var90 :bool;
    let mut varc0 :bool;
    let status_kind = fighter.global_table[0xb].get_i32();
    let status_kind_interrupt = fighter.global_table[0x9].get_i32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    let cat_3 = fighter.global_table[0x22].get_i32();
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_KAMUI {
        1
    }
    else {
        0
    };
    let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(fighter_manager,FighterEntryID(entry_id));
    let battle_object = *(entry as *mut u64).add(0x4160/8) as *mut u64;
    let enemy = *battle_object.add(0x20/8) as *mut BattleObjectModuleAccessor;
    if PostureModule::pos_x(enemy) > PostureModule::pos_x(fighter.module_accessor) {
        WorkModule::set_float(fighter.module_accessor, 1.0,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    }
    else {
        WorkModule::set_float(fighter.module_accessor, -1.0,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    }



    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    if !(lr_1on1 == 0.0) {
        if !(lr_1on1 == PostureModule::lr(fighter.module_accessor)) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                var90 = false;
                if !(status_kind == FIGHTER_STATUS_KIND_WALK) {
                    if  [
                        *FIGHTER_STATUS_KIND_SQUAT,
                        *FIGHTER_STATUS_KIND_SQUAT_RV,
                        *FIGHTER_STATUS_KIND_LANDING,
                        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
                        *FIGHTER_STATUS_KIND_GUARD_ON,
                        *FIGHTER_STATUS_KIND_ESCAPE,
                        *FIGHTER_STATUS_KIND_ATTACK_HI3,
                        *FIGHTER_STATUS_KIND_ATTACK_LW3,
                        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                        *FIGHTER_STATUS_KIND_CATCH,
                        *FIGHTER_STATUS_KIND_ITEM_SWING,
                        *FIGHTER_STATUS_KIND_SPECIAL_N,
                        *FIGHTER_STATUS_KIND_FINAL,
                        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
                    ].contains(&status_kind){
                        update_lr(fighter);
                        return 0.into();
                    }
                    if !(status_kind == *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR) {
                        if [
                            *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
                            *FIGHTER_RYU_STATUS_KIND_FINAL2
                        ].contains(&status_kind){
                                update_lr(fighter);
                                return 0.into();
                            if status_kind == *FIGHTER_STATUS_KIND_WAIT {
                                some_function(&varc0,status_kind_interrupt);
                                if varc0 {
                                    if var90 {
                                        update_lr(fighter);
                                    }
                                    return 0.into();
                                }
                                update_lr(fighter);
                                return 0.into();
                            }
                            if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                                if status_kind_interrupt == *FIGHTER_STATUS_KIND_TURN_RUN {
                                    if var90 {
                                        update_lr(fighter);
                                    }
                                    return 0.into();
                                }
                                update_lr(fighter);
                                return 0.into();
                            }
                            if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                                if status_kind_interrupt == *FIGHTER_STATUS_KIND_ATTACK {
                                    if !(ComboModule::count(fighter.module_accessor) == 0){
                                        if var90 {
                                            update_lr(fighter);
                                        }
                                        return 0.into();
                                    }
                                }
                                update_lr(fighter);
                                return 0.into();
                            }
                            if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW {
                                if situation_kind == *SITUATION_KIND_GROUND {
                                    if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 {
                                        if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4{
                                            if var90 {
                                                update_lr(fighter);
                                            }
                                            return 0.into();
                                        }
                                        var90 = true;
                                    } else {
                                        if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI {
                                            var90 = true;
                                        } else {
                                            if cat_3 !=*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW {
                                                if var90 {
                                                    update_lr(fighter);
                                                }
                                                return 0.into();
                                            }
                                            var90 = true;
                                        }
                                    }
                                    if var90 {
                                        update_lr(fighter);
                                    }
                                    return 0.into();
                                }
                            }
                            if var90 {
                                update_lr(fighter);
                            }
                            return 0.into();
                        }
                        varc0 = false;
                    } else {
                        varc0 = true;
                    }
                    if varc0 == true{ 
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_WAIT {
                        some_function(&varc0,status_kind_interrupt);
                        if varc0 {
                            if var90 {
                                update_lr(fighter);
                            }
                            return 0.into();
                        }
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                        if status_kind_interrupt == *FIGHTER_STATUS_KIND_TURN_RUN {
                            if var90 {
                                update_lr(fighter);
                            }
                            return 0.into();
                        }
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                        if status_kind_interrupt == *FIGHTER_STATUS_KIND_ATTACK {
                            if !(ComboModule::count(fighter.module_accessor) == 0){
                                if var90 {
                                    update_lr(fighter);
                                }
                                return 0.into();
                            }
                        }
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW {
                        if situation_kind == *SITUATION_KIND_GROUND {
                            if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 {
                                if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4{
                                    if var90 {
                                        update_lr(fighter);
                                    }
                                    return 0.into();
                                }
                                var90 = true;
                            } else {
                                if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI {
                                    var90 = true;
                                } else {
                                    if cat_3 !=*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW {
                                        if var90 {
                                            update_lr(fighter);
                                        }
                                        return 0.into();
                                    }
                                    var90 = true;
                                }
                            }
                            if var90 {
                                update_lr(fighter);
                            }
                            return 0.into();
                        }
                    }
                } else {
                    var90 = true;
                }
                if var90 {
                    update_lr(fighter);
                }
            }
        }
    }
    return 0.into();
}
