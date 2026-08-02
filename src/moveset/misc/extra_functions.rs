use super::*;


/*#[skyline::from_offset(0xb5f380)]
unsafe extern "C" fn kamui_link_event_original();

unsafe extern "C" fn kamui_link_event(vtable: u64, fighter: &mut Fighter) {
    println!("link event hook successful");
    kamui_init_original();
}*/

// arg 2 as 0 hides the stage and as 1 unhides it
#[skyline::from_offset(0x159fb20)]
pub unsafe extern "C" fn set_stage_visibility(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: bool);

#[skyline::from_offset(0x69a6e0)]
pub unsafe extern "C" fn get_opponent_lr(module_accessor: *mut smash::app::BattleObjectModuleAccessor, is_damage: bool, status_kind: i32, param_always_0: i32) -> f32;

#[skyline::from_offset(0x69ad00)]
pub unsafe extern "C" fn set_lr_float(lr_float: f32, module_accessor: *mut smash::app::BattleObjectModuleAccessor, status_kind: i32, param_always_0: i32) -> u64;
/*
#[skyline::from_offset(0x6955b0)]
pub unsafe extern "C" fn internal_final_cam(module: CameraModule, status_kind: Hash40, param3: i32) -> f32;
*/
#[skyline::from_offset(0x62a540)]
pub unsafe extern "C" fn kill_maybe(fighter: &mut Fighter, param_2: i32);




extern "C" {
    #[link_name = "\u{1}_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN3app16kiiladarzmanager16stop_stage_soundEv"]
    pub fn stop_stage_sound();
}
extern "C" {
    #[link_name = "\u{1}_ZN3app13kozukatasight14set_bgm_volumeEfj"]
    pub fn set_bgm_volume(param_1: f32, param_2: i32); //param 1 is probably volume and paramm 2 is probably fade in frames
}
//squid sisters shit
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu7end_bgmEii"]
    pub fn end_bgm(param_1:i32 ,param_2:i32); // param 2 is fade in frame but no clue what param 1 is
}
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu12get_bgm_kindEv"]
    pub fn get_bgm_kind();
}
extern "C" {
    #[link_name = "\u{1}_ZN3app10shiokarazu12get_warp_posEv"]
    pub fn get_warp_pos();
}
//sv_information





pub unsafe extern "C" fn some_function(status_kind: i32) -> bool {
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
        return true;
    } else {
        return false;
    }
}
    

unsafe extern "C" fn update_lr(fighter: &mut L2CFighterCommon) {
    let lr_1on1 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    //PostureModule::set_lr(fighter.module_accessor, lr_1on1);
    //PostureModule::update_rot_y_lr(fighter.module_accessor);
    //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN.into(),false.into());
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
                        FIGHTER_KAMUI_STATUS_KIND_WALK_BACK,
                        *FIGHTER_STATUS_KIND_ATTACK,
                    ].contains(&status_kind){
                        update_lr(fighter);
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_WAIT {
                        if !some_function(status_kind_interrupt) {
                            update_lr(fighter);
                        }
                        return 0.into();
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                        if status_kind_interrupt != *FIGHTER_STATUS_KIND_TURN_RUN {
                            update_lr(fighter);
                        }
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
                        if situation_kind == *SITUATION_KIND_GROUND {
                            if (cat_3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0) 
                            && (cat_3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 != 0) {
                                return 0.into();
                            } else if cat_3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI != 0 {
                                
                            } else if cat_3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW != 0{
                                return 0.into();
                            }
                            update_lr(fighter);
                            return 0.into();
                        }
                    }
                } else {
                    update_lr(fighter);
                }
            }
        }
    }
    return 0.into();
}
