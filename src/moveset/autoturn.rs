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




    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
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
                                if situation_kind == *SITUATION_KIND_GROUND
                                || situation_kind == *SITUATION_KIND_AIR {
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
                        if situation_kind == *SITUATION_KIND_GROUND
                        || situation_kind == *SITUATION_KIND_AIR {
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


pub unsafe extern "C" fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.status_pre_Wait_check_interrupt().get_bool() {
        return 1.into();
    }
    if !(WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == 0.0) {
        if !(WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == PostureModule::lr(fighter.module_accessor)) {
            let mut some_bool = false;
            let mut some_bool_2= false;
            some_function(&some_bool,fighter.global_table[PREV_STATUS_KIND].get_i32());
            if (some_bool == false) {
                if !(GroundModule::is_ottotto(fighter.module_accessor, 0.86)) {
                    some_bool_2 = false;
                } else {
                    if !(fighter.global_table[STICK_X].get_f32().abs() < 0.75) {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
                        return 1.into();
                    }
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO) {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
                        return 1.into();
                    }
                    some_bool_2 = true;
                }
                if some_bool_2 {
                    fighter.status_pre_Wait_main();
                    return 1.into();
                }
            }
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN);
            return 1.into();
        }
    }
    fighter.status_pre_Wait_main();
    return 1.into();
}


pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    fighter.sub_shift_status_main(L2CValue::Ptr(wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let lr = PostureModule::lr(fighter.module_accessor);
    if lr_1on1 != 0.0 {
        if (lr_1on1 == lr)
        && (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK))
        && (WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) <= (PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(module_accessor)).abs() )
        && (WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y")) <= fighter.global_table[STICK_Y].get_f32())
        {
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_WALK_BACK.into(),true.into());
            return 0.into();
        }else if (lr_1on1 != lr){
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
            return 0.into();
        }
    }
    return 0.into();
}


pub unsafe extern "C" fn sub_walk_uniq_check(fighter: &mut L2CFighterCommon) {
    fighter.sub_walk_uniq_check();
}




pub unsafe extern "C" fn walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.status_Walk_Common();
    if StopModule::is_stop(fighter.module_accessor) == false {
        fighter.sub_walk_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(sub_walk_uniq_check as *const () as _));   
    fighter.sub_shift_status_main(L2CValue::Ptr(walk_main_loop as *const () as _))
}



pub unsafe extern "C" fn walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.status_Walk_Main_common((*FIGHTER_STATUS_KIND_WALK_BRAKE).into(), (fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor)).into(), true.into(), L2CValue::Ptr(walk_function_idfk as *const () as _));

    return 0.into();
}


pub unsafe extern "C" fn walk_function_idfk(fighter: &mut L2CFighterCommon) -> bool{
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != 0.0 )
    && (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != PostureModule::lr(module_accessor)) {
        fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
        return true.into()
    }
    return false.into()
}




pub unsafe extern "C" fn walkback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk_Common();
    if StopModule::is_stop(fighter.module_accessor) == false {
        fighter.sub_walk_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(sub_walk_uniq_check as *const () as _));   
    fighter.sub_shift_status_main(L2CValue::Ptr(walkback_main_loop as *const () as _))
}

pub unsafe extern "C" fn walkback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.status_Walk_Main_common((*FIGHTER_STATUS_KIND_WALK_BRAKE).into(), (fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor)).into(), true.into(), L2CValue::Ptr(walk_function_idfk as *const () as _));
    fighter.status_Walk_Main_common(
        (FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK).into(),
        (-PostureModule::lr(fighter.module_accessor) * fighter.global_table[STICK_X].get_f32()).into(),
        false.into(),
        L2CValue::Ptr(walk_function_idfk as *const () as _)
    );
    return 0.into();
}

pub unsafe extern "C" fn walkback_calc_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.FighterStatusUniqProcessWalkBack_calc_param();
    return 0.into();
}

pub unsafe extern "C" fn walkback_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_exec();
    return 0.into();
}
pub unsafe extern "C" fn walkback_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_init();
    return 0.into();
}
pub unsafe extern "C" fn walkback_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_back_uniq_process_exit();
    return 0.into();
}
pub unsafe extern "C" fn walkback_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_WalkBack();
    return 0.into();
}
pub unsafe extern "C" fn walkback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_WalkBack();
    return 0.into();
}



pub unsafe extern "C" fn autoturn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait();
    return 0.into();
}

pub unsafe extern "C" fn autoturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    PostureModule::reverse_lr(fighter.module_accessor);
    if ControlModule::is_remake_command(fighter.module_accessor) {
        fighter.sub_fighter_remake_control_command();
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(autoturn_main_loop as *const () as _))
}

pub unsafe extern "C" fn autoturn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_wait_common_Main().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return 0.into();
}

pub unsafe extern "C" fn autoturn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Wait();
    return 0.into();
}


pub unsafe extern "C" fn walk_back_brake_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_brake_uniq_process_exec();
    return 0.into();
}
pub unsafe extern "C" fn walk_back_brake_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_brake_uniq_process_init();
    return 0.into();
}
pub unsafe extern "C" fn walk_back_brake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_WalkBrake();
    return 0.into();
}
pub unsafe extern "C" fn walk_back_brake_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_WalkBrakeBack();
    return 0.into();
}
pub unsafe extern "C" fn walk_back_brake_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_WalkBrake();
    return 0.into();
}






/*
        // #[null_check]
        #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_execEv"]
        pub fn L2CFighterCommon_sub_walk_back_uniq_process_exec(
            this: *mut crate::common::root::lua2cpp::L2CFighterCommon
        );


        // #[null_check]
        #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_exitEv"]
        pub fn L2CFighterCommon_sub_walk_back_uniq_process_exit(
            this: *mut crate::common::root::lua2cpp::L2CFighterCommon
        );


        // #[null_check]
        #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_initEv"]
        pub fn L2CFighterCommon_sub_walk_back_uniq_process_init(
            this: *mut crate::common::root::lua2cpp::L2CFighterCommon
        );


        // #[null_check]
        #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessWalkBack_calc_paramEv"]
        pub fn L2CFighterCommon_FighterStatusUniqProcessWalkBack_calc_param(
            this: *mut crate::common::root::lua2cpp::L2CFighterCommon
        );


        // #[null_check]
        #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon22status_WalkBack_commonEv"]
        pub fn L2CFighterCommon_status_WalkBack_common(
            this: *mut crate::common::root::lua2cpp::L2CFighterCommon
        );

        #[inline]
        pub unsafe fn sub_walk_back_uniq_process_exec(&mut self) {
            L2CFighterCommon_sub_walk_back_uniq_process_exec(self)
        }
        #[inline]
        pub unsafe fn sub_walk_back_uniq_process_exit(&mut self) {
            L2CFighterCommon_sub_walk_back_uniq_process_exit(self)
        }
        #[inline]
        pub unsafe fn sub_walk_back_uniq_process_init(&mut self) {
            L2CFighterCommon_sub_walk_back_uniq_process_init(self)
        }
        #[inline]
        pub unsafe fn FighterStatusUniqProcessWalkBack_calc_param(&mut self) {
            L2CFighterCommon_FighterStatusUniqProcessWalkBack_calc_param(self)
        }
        #[inline]
        pub unsafe fn status_WalkBack_common(&mut self) {
            L2CFighterCommon_status_WalkBack_common(self)
        }
*/