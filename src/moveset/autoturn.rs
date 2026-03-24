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
        *FIGHTER_STATUS_KIND_FINAL
        ,*FIGHTER_STATUS_KIND_SPECIAL_N
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_JUMP_END
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END
        ,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP
        ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START
        ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP
        ,FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE
        ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE
        ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD
        ,FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END
        ,FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER
        ,FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF
        ,FIGHTER_KAMUI_STATUS_KIND_AIR_THROW
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
    //PostureModule::set_lr(fighter.module_accessor, lr_1on1);
    //PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
    //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
}



pub unsafe extern "C" fn auto_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut var90 :bool;
    let mut varc0 :bool;
    let status_kind = fighter.global_table[0xb].get_i32();
    let status_kind_interrupt = fighter.global_table[0x9].get_i32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    let cat_3 = fighter.global_table[0x22].get_i32();


    //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if !(lr_1on1 == 0.0) {
        if !(lr_1on1 == PostureModule::lr(fighter.module_accessor)) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                var90 = false;
                varc0 = false;
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
                        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
                        *FIGHTER_STATUS_KIND_ATTACK,
                    ].contains(&status_kind){
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_WAIT {
                        some_function(&varc0,status_kind_interrupt);
                        if varc0 {
                            return 0.into();
                        }
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                        if status_kind_interrupt == *FIGHTER_STATUS_KIND_TURN_RUN {
                            return 0.into();
                        }
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                        if status_kind_interrupt == *FIGHTER_STATUS_KIND_ATTACK {
                            if !(ComboModule::count(fighter.module_accessor) == 0){
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
                                    return 0.into();
                                }
                            } else {
                                if cat_3 == *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI {
                                } else {
                                    if cat_3 !=*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW {
                                        return 0.into();
                                    }
                                }
                            }
                            update_lr(fighter);
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
    if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_KAMUI_STATUS_KIND_WALK_BACK{
        MotionModule::change_motion(module_accessor,Hash40::new("walk_stop_b"),0.0,1.0,false,0.0,false,false);
    }else if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_WALK{
        MotionModule::change_motion(module_accessor,Hash40::new("walk_stop_f"),0.0,1.0,false,0.0,false,false);
    }
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

pub unsafe extern "C" fn turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) != 0.0 )
    && (WorkModule::get_float(module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1) == PostureModule::lr(module_accessor)) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, FIGHTER_KAMUI_STATUS_KIND_WALK_BACK);
        return 1.into();
    }
    fighter.status_pre_Turn();
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

pub unsafe extern "C" fn air_autoturn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Fall();
    return 0.into();
}

pub unsafe extern "C" fn air_autoturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Fall();
    PostureModule::reverse_lr(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(air_autoturn_main_loop as *const () as _))
}

pub unsafe extern "C" fn air_autoturn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Fall_Main();
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FALL{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }else if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FALL_AERIAL{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(),false.into());
        }else{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    
    return 0.into();
}

pub unsafe extern "C" fn air_autoturn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Fall();
    return 0.into();
}


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

pub unsafe extern "C" fn fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.sub_fall_uniq_process_init();
    if FIGHTER_KAMUI_STATUS_KIND_WALK_BACK == fighter.global_table[PREV_STATUS_KIND] {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_FALL_FLAG_FROM_GROUND);
    }
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



pub unsafe extern "C" fn pass_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    println!("pass exec");
    fighter.sub_uniq_process_Pass_exec_status();
    if WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
        println!("less than 15");
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        let offset_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_OFFSET_Y);
        let air_accel_y = -WorkModule::get_param_float(module_accessor, hash40("air_accel_y"),0);
        println!("air accel y = {}",air_accel_y);
        let offset = air_accel_y + offset_y;
        println!("offset = {}",offset);
        GroundModule::set_offset_y(module_accessor, offset);
        WorkModule::set_float(module_accessor, offset, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        WorkModule::set_float(module_accessor, offset, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_OFFSET_Y);
        WorkModule::dec_int(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
    }else{
        println!("more than 15");
    }

    return L2CValue::I32(0)
}


pub unsafe extern "C" fn pass_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    GroundModule::set_offset_y(module_accessor, 0.0);
    if WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
        println!("less than 15 end");

        fighter.end_pass_ground();
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    fighter.status_end_Pass();
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn pass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.status_Pass_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(pass_main_loop as *const () as _))
}


pub unsafe extern "C" fn pass_function_thingy(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.sub_transition_group_check_special_command().get_bool(){
        WorkModule::off_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
    }else{
        if !WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
            let value = 8; //WorkModule::get_param_float(module_accessor, hash40("param_private"),hash40("pass_cancel_command_frame"));
            WorkModule::set_int(module_accessor, value, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
            WorkModule::on_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
            //return L2CValue::I32(1)
        }
        let frame = WorkModule::get_int(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
        if !(frame == 0) {
            return L2CValue::I32(1);
        }
        GroundModule::set_offset_y(module_accessor, 0.0);
        WorkModule::off_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
        fighter.end_pass_ground();
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        return L2CValue::I32(0);
    }
    return L2CValue::I32(1);
}


pub unsafe extern "C" fn pass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Pass_Main_sub(L2CValue::Ptr(pass_function_thingy as *const () as _));
    return L2CValue::I32(0)
}


