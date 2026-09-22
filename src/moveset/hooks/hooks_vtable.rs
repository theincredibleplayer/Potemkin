use super::*;
use std::sync::LazyLock;

static KAMUI_ON_RESET_OFFSET            : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::RESET_1                   , false, false));
static KAMUI_INIT_OFFSET                : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::INITIALIZE                , false, false));
static KAMUI_ON_SHIELD                  : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_SHIELD_COLLISION       , false, false));
static KAMUI_ON_DAMAGE                  : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_DAMAGE                 , false, false));
static KAMUI_ON_ATTACK                  : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_ATTACK                 , false, false));
static KAMUI_ON_CHANGE_MOTION           : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_CHANGE_MOTION          , false, false));
static KAMUI_ON_SITUATION_CHANGE_2      : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_SITUATION_CHANGE_2     , false, false));
static KAMUI_ON_STATUS_CHANGE_2         : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_STATUS_CHANGE_2        , false, false));
static KAMUI_ON_DEATH                   : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_DEATH                  , false, false));

static OFFSET_ON_FRAME_4                : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_FRAME_4                , false, true));
static OFFSET_ON_SEARCH                 : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_SEARCH                 , false, true));
static OFFSET_RESET_1                   : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::RESET_1                   , false, true));
static OFFSET_INITIALIZE                : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::INITIALIZE                , false, true));
static OFFSET_ON_SHIELD_COLLISION       : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_SHIELD_COLLISION       , false, true));
static OFFSET_ON_DAMAGE                 : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_DAMAGE                 , false, true));
static OFFSET_ON_ATTACK                 : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_ATTACK                 , false, true));
static OFFSET_ON_CHANGE_MOTION          : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_CHANGE_MOTION          , false, true));
static OFFSET_ON_STATUS_CHANGE          : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_STATUS_CHANGE          , false, true));
static OFFSET_ON_SITUATION_CHANGE_2     : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_SITUATION_CHANGE_2     , false, true));
static OFFSET_ON_STATUS_CHANGE_2        : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_STATUS_CHANGE_2        , false, true));
static OFFSET_ON_DEATH                  : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*FIGHTER_KIND_KAMUI, VtableEntry::ON_DEATH                  , false, true));

//searchbox implementations
pub unsafe extern "C" fn kamui_on_search(_vtable: u64, fighter: &mut Fighter, log: u64, _some_float: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    if is_potemkin(module_accessor){
        if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL {
            let collision_log = *(log as *const u64).add(0x10 / 0x8);
            let collision_log = collision_log as *const CollisionLogScuffed;
            let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
            StatusModule::change_status_request(module_accessor,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,false);
        }
        if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL {
            let collision_log = *(log as *const u64).add(0x10 / 0x8);
            let collision_log = collision_log as *const CollisionLogScuffed;
            let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
            let clear_time = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME);
            let slow_mul = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL) as i32;    
            let stop_time = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION);


            let is_opponent_clock = WorkModule::is_flag(opp_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SLOW_CAUSE_TIMER);
            let is_opponent_clock_whole = WorkModule::is_flag(opp_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_WHOLE_SLOW_CAUSE_TIMER);
            let do_slow = !(is_opponent_clock || is_opponent_clock_whole);
    
            if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_END_SLOW){
                // the 0x800000 seems to make the effect immune to the user being slowed
                // also apparently the order of spawning the effects matter in the draw order?
                EffectModule::req_follow(opp_boma, Hash40::new("rc_clock_base"), Hash40::new("top"), &Vector3f{x: 0.0, y: 23.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.3, true, 0x800000, 0, -1, 0, 0, false, false);
                EffectModule::set_rate_last(opp_boma, ( ( 60.0 )/( clear_time as f32 ) ) );
                EffectModule::req_follow(opp_boma, Hash40::new("rc_clock"), Hash40::new("top"), &Vector3f{x: 0.0, y: 23.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.3, true, 0x800000, 0, -1, 0, 0, false, false);
                EffectModule::set_rate_last(opp_boma, ( ( 60.0 )/( clear_time as f32 ) ) );

                if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC){
                    WorkModule::set_float(opp_boma, 0.25, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                    if StatusModule::situation_kind(opp_boma) != *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(opp_boma, FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, false);
                    }else{
                        StatusModule::change_status_request_from_script(opp_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                    }
                    if do_slow{
                        SlowModule::set(
                            opp_boma,
                            0,
                            slow_mul,
                            clear_time,
                            false,
                            *BATTLE_OBJECT_ID_INVALID as u32
                        );
                    }
                }else{
                    if do_slow{
                        SlowModule::set(
                            opp_boma,
                            0,
                            slow_mul,
                            clear_time,
                            false,
                            *BATTLE_OBJECT_ID_INVALID as u32
                        );
                    }
                }
            }else if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_INITIAL_SLOW){
                if do_slow{
                    SlowModule::set(
                        opp_boma,
                        0,
                        255,
                        stop_time,
                        false,
                        *BATTLE_OBJECT_ID_INVALID as u32
                    );
                }
            }
        }
        if StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_FINAL {
            let collision_log = *(log as *const u64).add(0x10 / 0x8);
            let collision_log = collision_log as *const CollisionLogScuffed;
            let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
            WorkModule::on_flag(module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, false);
        }
    }
}


#[skyline::from_offset(0x69ae40)]
extern "C" fn ryu_ken_autoturn_handler(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32, param_2: u32);

pub unsafe extern "C" fn kamui_on_frame(_vtable: u64, fighter: &mut Fighter) {
    let scale = PostureModule::scale(fighter.battle_object.module_accessor); //gets the model's scale
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind =StatusModule::status_kind(fighter.battle_object.module_accessor) ;
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let motion_kind =MotionModule::motion_kind(module_accessor);


    if is_potemkin(module_accessor){
        //let prevent_turn = StopModule::is_damage(module_accessor) as u32;
        //ryu_ken_autoturn_handler(module_accessor,prevent_turn,0);
        potemkin_mirror_handler(module_accessor);
        //weapon attaching code
        potemkin_weapon_attach_handler(module_accessor);
        // armor effect
        let time_base = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE);
        let agent = get_fighter_common_from_accessor((&mut*module_accessor));
        let fire_timer = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
        if (fire_timer > 0) {
            WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
            let fire_timer = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
            macros::BURN_COLOR_FRAME(agent, 0, 24, 1, 1, 0);
            macros::BURN_COLOR(agent, 24.0 * ( time_base/((fire_timer as f32)+1.0) ),  1.0 * ( time_base/((fire_timer as f32)+1.0) ),  1.0 * ( time_base/((fire_timer as f32)+1.0) ), (0.5/time_base)*(fire_timer as f32));
        }

        // ui stuff
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_manager = singletons::FighterManager();
        if smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) {
            UiManager::disable_ui(get_ui_index_from_entry_id(entry_id));
        }else{
            UiManager::enable_ui(get_ui_index_from_entry_id(entry_id));
        }
        UiManager::set_game_speed(sv_information::slow_rate()); 

        //post_effect line
        if WorkModule::is_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SET_ADJUST_POST_CIRCLE){
            let agent = get_fighter_common_from_accessor((&mut*module_accessor));
            let radius = 20.0;
            let vec1 = simd::Vector2{vec: [radius /* radius x */, radius /* radius y */]}; 
            let vec2 = simd::Vector3{vec: [0.0 /* y offset */, 0.0 /* x offset */, 0.0 /* z offset */]}; 
            EffectModule::set_post_effect_line_circle_target(module_accessor,Hash40::new("hip"),vec1,vec2,true);
        }


        // forbids dashing
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);


        // autoturn value
        let is_damage = StopModule::is_damage(module_accessor);

        let lr = get_opponent_lr(module_accessor,is_damage,status_kind,0);
        set_lr_float(lr,module_accessor,status_kind,0);


        //air turnaround
        let lr_1on1 = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
        let lr = PostureModule::lr(module_accessor);
        if lr_1on1 != 0.0 {
            if situation_kind == *SITUATION_KIND_AIR
            && MotionModule::motion_kind(module_accessor) != hash40("air_turn")
            && lr_1on1 != lr
            && [
                *FIGHTER_STATUS_KIND_JUMP,
                *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                *FIGHTER_STATUS_KIND_FALL,
                *FIGHTER_STATUS_KIND_FALL_AERIAL,
                *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
                *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
                *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
            ].contains(&status_kind){
                StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN ,false);
            }
        }


    }
}


// initialization vtable hook (using it as agent init since i cant use workmodule in agent init) (thanks wuboy)
#[skyline::from_offset(*KAMUI_ON_RESET_OFFSET)]
pub unsafe extern "C" fn kamui_reset_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_reset(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let item_module = *(module_accessor as *const *const u64).add(0xc8 / 8);
    let mut item_hash_ptr = (item_module as u64 + 0x20) as *mut *mut u64 ;
    let mut item_hash_ptr_2 = (*item_hash_ptr as u64 + 0x8) as *mut u64;
    let mut genesis_bone = (*item_hash_ptr as u64 + 0x18) as *mut u64;
    let dat = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x4fc8fe0) as u64;
    let mut commonhave_hash = (dat + 0x20) as  *mut *mut u64;

    
    if utility::get_kind(&mut *module_accessor) == *FIGHTER_KIND_KAMUI{
        if is_potemkin_svinfo(module_accessor){
            **commonhave_hash  = hash40("haver");
            **item_hash_ptr    = hash40("haver");
            *(item_hash_ptr_2) = hash40("haver");
            *(genesis_bone)    = hash40("g_grip_r");
        } else {
            **commonhave_hash  = hash40("havel");
            **item_hash_ptr    = hash40("havel");
            *(item_hash_ptr_2) = hash40("havel");
            *(genesis_bone)    = hash40("havel");
        }
    }


    if is_potemkin_svinfo(module_accessor){
        ShieldModule::set_target_property(fighter.battle_object.module_accessor, *COLLISION_PROPERTY_REFLECT,2);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        UiManager::reset_on_rebirth(get_ui_index_from_entry_id(entry_id));
        UiManager::reset_cutin();
        if smashball::is_training_mode(){
            UiManager::add_tension_unscaled(get_ui_index_from_entry_id(entry_id),150.0);
        }
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);

        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP);
        WorkModule::on_flag(module_accessor ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
        WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
        WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
        WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);



        let module_accessor_2 = fighter.battle_object.module_accessor;
        let control_energy = KineticModule::get_energy(module_accessor_2, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        *(control_energy as *mut u8).add(0xa4) = 1;

        //let kinetic_module = *(module_accessor_2 as *const *const u64).add(0x68 / 8);
        //let func_addr1 = *((*(kinetic_module) + 0x60)as *mut u64);
        //let get_kinetic_energy: extern "C" fn(*const u64, i32, ...) -> u64 = std::mem::transmute(func_addr1);
        //let kinetic_energy = get_kinetic_energy(kinetic_module,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        //*(kinetic_energy as *mut u8).add(0xa4) = 1;

    }
    kamui_reset_original(vtable,fighter);
}

#[skyline::from_offset(*KAMUI_ON_RESET_OFFSET)]
pub unsafe extern "C" fn kamui_on_change_motion_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_change_motion(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let motion = MotionModule::motion_kind(module_accessor);
        potemkin_mirror_handler(module_accessor);
        if [
            hash40("wait"),
            hash40("wait_2"),
            hash40("wait_3"),
            hash40("walk_stop_b"),
            hash40("walk_stop_f"),
            hash40("wait_item"),
            hash40("item_hand_pickup"),
            hash40("item_hand_have"),
            hash40("item_hand_grip"),
            hash40("item_hand_smash"),
            hash40("walk_slow"),
            hash40("walk_middle"),
            hash40("walk_fast"),
            hash40("walk_brake_r"),
            hash40("walk_brake_l"),
            hash40("walk_slow_b"),
            hash40("walk_middle_b"),
            hash40("walk_fast_b"),
            hash40("walk_brake_r_b"),
            hash40("walk_brake_l_b"),
            hash40("turn"),
            hash40("air_turn"),
            hash40("squat_turn"),
            hash40("dash"),
            hash40("turn_dash"),
            hash40("run"),
            hash40("run_brake_r"),
            hash40("run_brake_l"),
            hash40("turn_run"),
            hash40("turn_run_brake"),
            hash40("jump_squat"),
            hash40("jump_f"),
            hash40("jump_b"),
            hash40("jump_f_mini"),
            hash40("jump_b_mini"),
            hash40("jump_aerial_f"),
            hash40("jump_aerial_b"),
            hash40("fall"),
            hash40("fall_f"),
            hash40("fall_b"),
            hash40("fall_aerial"),
            hash40("fall_aerial_f"),
            hash40("fall_aerial_b"),
            hash40("fall_special"),
            hash40("fall_special_f"),
            hash40("fall_special_b"),
            hash40("landing_fall_special"),
            hash40("fall_damage"),
            hash40("walk_fall_r"),
            hash40("walk_fall_l"),
            //hash40("run_fall_r"),
            //hash40("run_fall_l"),
            hash40("squat"),
            hash40("squat_wait"),
            hash40("squat_wait_item"),
            hash40("squat_rv"),
            hash40("landing_light"),
            hash40("landing_heavy"),
            hash40("step_jump"),
            hash40("step_pose"),
            hash40("step_pose_back"),
            hash40("step_pose_air"),
            hash40("step_fall"),
            hash40("guard_on"),
            hash40("guard"),
            hash40("guard_off"),
            hash40("just_shield_off"),
            hash40("guard_damage"),
            hash40("escape"),
            hash40("escape_f"),
            hash40("escape_b"),
            hash40("escape_air"),
            hash40("escape_air_slide"),
            hash40("rebound"),
            hash40("attack_11"),
            //hash40("attack_12"),
            //hash40("attack_13"),
            //hash40("attack_100_start"),
            //hash40("attack_100"),
            //hash40("attack_100_end"),
            hash40("attack_dash"),
            hash40("attack_s3_s"),
            hash40("attack_hi3"),
            hash40("attack_lw3"),
            hash40("attack_s4_hi"),
            hash40("attack_s4_s"),
            hash40("attack_s4_lw"),
            hash40("attack_s4_hold"),
            hash40("attack_hi4"),
            hash40("attack_hi4_hold"),
            hash40("attack_lw4"),
            hash40("attack_lw4_hold"),
            hash40("attack_air_n"),
            hash40("attack_air_f"),
            hash40("attack_air_b"),
            hash40("attack_air_hi"),
            hash40("attack_air_lw"),
            hash40("landing_air_n"),
            hash40("landing_air_f"),
            hash40("landing_air_b"),
            hash40("landing_air_hi"),
            hash40("landing_air_lw"),
            hash40("special_n_start"),
            hash40("special_n_hold"),
            hash40("special_n_fire"),
            hash40("special_n_end_1"),
            hash40("special_n_end_2"),
            hash40("special_air_n_start"),
            hash40("special_air_n_hold"),
            hash40("special_air_n_fire"),
            hash40("special_air_n_end_1"),
            hash40("special_air_n_end_2"),
            hash40("special_s_jump"),
            hash40("special_s_jump_end"),
            hash40("special_s_attack"),
            hash40("special_air_s_attack"),
            hash40("special_s_attack_end"),
            hash40("special_s_wall"),
            hash40("special_s_wall_end"),
            hash40("special_s_wall_jump"),
            hash40("special_s_wall_attack_f"),
            hash40("special_s_wall_attack_b"),
            hash40("special_s_attack_f_landing"),
            hash40("special_s_attack_b_landing"),
            hash40("special_s"),
            hash40("special_s_slide"),
            hash40("special_s_end"),
            hash40("special_s_attack"),
            hash40("special_s_mf_f"),
            hash40("special_s_mf_f_land"),
            hash40("special_s_mf_b"),
            hash40("special_s_mf_b_land"),
            hash40("special_hi"),
            hash40("special_hi_start"),
            hash40("special_hi_loop"),
            hash40("special_hi_release"),
            hash40("special_air_hi"),
            hash40("special_air_hi_up"),
            hash40("special_air_hi_hold"),
            hash40("special_air_hi_hold_ground"),
            hash40("special_air_hi_end"),
            hash40("special_lw"),
            hash40("special_lw_hit"),
            hash40("special_lw_hit_turn"),
            hash40("special_air_lw"),
            hash40("special_air_lw_hit"),
            hash40("special_air_lw_hit_turn"),
            hash40("potemkin_buster"),
            hash40("potemkin_buster_whiff"),
            hash40("roman_cancel"),
            hash40("roman_cancel_air"),
            hash40("final"),
            hash40("final_air"),
            hash40("heavenly_dash_start"),
            hash40("heavenly_dash_loop"),
            hash40("heavenly_dash_attack"),
            hash40("heavenly_start"),
            hash40("final_visual_scene_01"),
            hash40("fake_final_visual_scene_01"),
            hash40("final_visual_scene_02"),
            hash40("final_visual_scene_03"),
            hash40("final_end"),
            hash40("final_air_end"),
            hash40("catch"),
            hash40("catch_dash"),
            hash40("catch_turn"),
            hash40("catch_pull"),
            hash40("catch_wait"),
            hash40("catch_attack"),
            hash40("catch_cut"),
            hash40("throw_f"),
            hash40("throw_b"),
            hash40("throw_hi"),
            hash40("throw_lw"),
            hash40("throw_air"),
            //hash40("thrown_f"),
            //hash40("thrown_b"),
            //hash40("thrown_hi"),
            //hash40("thrown_lw"),
            //hash40("thrown_f_dx"),
            //hash40("thrown_b_dx"),
            //hash40("thrown_hi_dx"),
            //hash40("thrown_lw_dx"),
            //hash40("thrown_f_big"),
            //hash40("thrown_b_big"),
            //hash40("thrown_hi_big"),
            //hash40("thrown_lw_big"),
            //hash40("thrown_f_girl"),
            //hash40("thrown_b_girl"),
            //hash40("thrown_hi_girl"),
            //hash40("thrown_lw_girl"),
            hash40("capture_pulled_hi"),
            hash40("capture_wait_hi"),
            hash40("capture_damage_hi"),
            hash40("capture_pulled_lw"),
            hash40("capture_wait_lw"),
            hash40("capture_damage_lw"),
            hash40("capture_cut"),
            hash40("capture_jump"),
            hash40("entry_r"),
            hash40("entry_l"),
            hash40("appeal_hi_r"),
            hash40("appeal_hi_l"),
            hash40("appeal_s_r"),
            hash40("appeal_s_l"),
            hash40("appeal_lw_r"),
            hash40("appeal_lw_l"),
            hash40("win_1"),
            hash40("win_1_wait"),
            hash40("win_2"),
            hash40("win_2_wait"),
            hash40("win_3"),
            hash40("win_3_wait"),
            hash40("lose"),
            hash40("damage_hi_1"),
            hash40("damage_hi_2"),
            hash40("damage_hi_3"),
            hash40("damage_n_1"),
            hash40("damage_n_2"),
            hash40("damage_n_3"),
            hash40("damage_lw_1"),
            hash40("damage_lw_2"),
            hash40("damage_lw_3"),
            hash40("damage_air_1"),
            hash40("damage_air_2"),
            hash40("damage_air_3"),
            hash40("damage_fly_hi"),
            hash40("damage_fly_n"),
            hash40("damage_fly_lw"),
            hash40("damage_fly_top"),
            hash40("damage_fly_roll"),
            hash40("damage_fly_roll_end"),
            hash40("damage_fly_meteor"),
            hash40("damage_elec"),
            hash40("down_bound_u"),
            hash40("down_wait_u"),
            hash40("down_damage_u"),
            hash40("down_damage_u3"),
            //hash40("down_eat_u"),
            hash40("down_stand_u"),
            hash40("down_attack_u"),
            hash40("down_forward_u"),
            hash40("down_back_u"),
            hash40("down_bound_d"),
            hash40("down_wait_d"),
            hash40("down_damage_d"),
            hash40("down_damage_d3"),
            //hash40("down_eat_d"),
            hash40("down_stand_d"),
            hash40("down_attack_d"),
            hash40("down_forward_d"),
            hash40("down_back_d"),
            hash40("down_spot_d"),
            hash40("passive"),
            hash40("passive_stand_f"),
            hash40("passive_stand_b"),
            hash40("passive_wall"),
            hash40("passive_wall_jump"),
            hash40("passive_ceil"),
            hash40("shield_break_fly"),
            hash40("furafura"),
            hash40("bind"),
            hash40("furafura_start_u"),
            hash40("furafura_start_d"),
            hash40("furafura_end"),
            hash40("fura_sleep_start"),
            hash40("fura_sleep_loop"),
            hash40("fura_sleep_end"),
            //hash40("swallowed"),
            hash40("camera_hit"),
            hash40("pass"),
            hash40("ottotto"),
            hash40("ottotto_wait"),
            hash40("wall_damage"),
            hash40("stop_wall"),
            hash40("ceil_damage"),
            hash40("stop_ceil"),
            hash40("miss_foot"),
            hash40("cliff_catch"),
            hash40("cliff_wait"),
            hash40("cliff_climb_quick"),
            hash40("cliff_attack_quick"),
            hash40("cliff_escape_quick"),
            hash40("cliff_jump_quick1"),
            hash40("cliff_jump_quick2"),
            hash40("slip_down"),
            hash40("slip"),
            hash40("slip_wait"),
            hash40("slip_stand"),
            hash40("slip_attack"),
            hash40("slip_escape_f"),
            hash40("slip_escape_b"),
            hash40("item_light_get"),
            hash40("item_light_walk_get"),
            hash40("item_light_eat"),
            hash40("item_light_walk_eat"),
            hash40("item_light_throw_f"),
            hash40("item_light_throw_b"),
            hash40("item_light_throw_hi"),
            hash40("item_light_throw_lw"),
            hash40("item_light_throw_f4"),
            hash40("item_light_throw_b4"),
            hash40("item_light_throw_hi4"),
            hash40("item_light_throw_lw4"),
            //hash40("item_light_throw_dash"),
            hash40("item_light_throw_drop"),
            hash40("item_light_throw_air_f"),
            hash40("item_light_throw_air_b"),
            hash40("item_light_throw_air_hi"),
            hash40("item_light_throw_air_lw"),
            hash40("item_light_throw_air_f4"),
            hash40("item_light_throw_air_b4"),
            hash40("item_light_throw_air_hi4"),
            hash40("item_light_throw_air_lw4"),
            hash40("item_heavy_wait"),
            hash40("item_heavy_get"),
            hash40("item_heavy_walk"),
            hash40("item_heavy_throw_f"),
            hash40("item_heavy_throw_b"),
            hash40("item_heavy_throw_hi"),
            hash40("item_heavy_throw_lw"),
            hash40("item_heavy_throw_f4"),
            hash40("item_heavy_throw_b4"),
            hash40("item_heavy_throw_hi4"),
            hash40("item_heavy_throw_lw4"),
            hash40("item_sword_swing_1"),
            hash40("item_sword_swing_3"),
            hash40("item_sword_swing_4"),
            hash40("item_sword_swing_4_hold"),
            //hash40("item_sword_swing_dash"),
            hash40("item_starrod_swing_1"),
            hash40("item_starrod_swing_3"),
            hash40("item_starrod_swing_4"),
            hash40("item_starrod_swing_4_hold"),
            //hash40("item_starrod_swing_dash"),
            hash40("item_lipstick_swing_1"),
            hash40("item_lipstick_swing_3"),
            hash40("item_lipstick_swing_4"),
            hash40("item_lipstick_swing_4_hold"),
            //hash40("item_lipstick_swing_dash"),
            hash40("item_firebar_swing_1"),
            hash40("item_firebar_swing_3"),
            hash40("item_firebar_swing_4"),
            hash40("item_firebar_swing_4_hold"),
            //hash40("item_firebar_swing_dash"),
            hash40("item_club_swing_1"),
            hash40("item_club_swing_3"),
            hash40("item_club_swing_4"),
            hash40("item_club_swing_4_hold"),
            //hash40("item_club_swing_dash"),
            hash40("item_killsword_swing_1"),
            hash40("item_killsword_swing_3"),
            hash40("item_killsword_swing_4"),
            hash40("item_killsword_swing_4_hold"),
            //hash40("item_killsword_swing_dash"),
            hash40("item_deathscythe_swing_1"),
            hash40("item_deathscythe_swing_3"),
            hash40("item_deathscythe_swing_4"),
            hash40("item_deathscythe_swing_4_hold"),
            //hash40("item_deathscythe_swing_dash"),
            //hash40("item_golden_hammer_wait"),
            //hash40("item_golden_hammer_move"),
            //hash40("item_golden_hammer_jump"),
            //hash40("item_hammer_wait"),
            //hash40("item_hammer_move"),
            //hash40("item_hammer_jump"),
            hash40("item_bat_swing_4"),
            hash40("item_bat_swing_1"),
            hash40("item_bat_swing_3"),
            //hash40("item_bat_swing_dash"),
            hash40("item_screw"),
            hash40("item_screw_air"),
            hash40("item_screw_fall"),
            hash40("barrel_screw"),
            //hash40("item_dragoon_get"),
            //hash40("item_dragoon_ride"),
            //hash40("item_warpstar"),
            hash40("item_shoot_legs_walk_f"),
            hash40("item_shoot_legs_walk_b"),
            hash40("item_shoot_legs_jump_squat"),
            hash40("item_shoot_legs_landing"),
            hash40("item_lgun_shoot"),
            hash40("item_lgun_shoot_upper"),
            hash40("item_lgun_shoot_air"),
            hash40("item_lgun_shoot_empty"),
            hash40("item_lgun_shoot_upper_empty"),
            hash40("item_lgun_shoot_air_empty"),
            hash40("item_fflower_shoot"),
            hash40("item_fflower_shoot_upper"),
            hash40("item_fflower_shoot_air"),
            hash40("item_fflower_shoot_aerial"),
            hash40("item_fflower_shoot_loop"),
            hash40("item_fflower_shoot_loop_upper"),
            hash40("item_fflower_shoot_air_loop"),
            hash40("item_scope_start"),
            hash40("item_scope_start_upper"),
            hash40("item_scope_wait"),
            hash40("item_scope_wait_upper"),
            hash40("item_scope_rapid"),
            hash40("item_scope_rapid_upper"),
            hash40("item_scope_fire"),
            hash40("item_scope_fire_upper"),
            hash40("item_scope_end"),
            hash40("item_scope_end_upper"),
            hash40("item_scope_fall"),
            hash40("item_scope_air_start"),
            hash40("item_scope_air_rapid"),
            hash40("item_scope_air_fire"),
            hash40("item_scope_air_end"),
            hash40("item_scope_air_start_aerial"),
            hash40("item_scope_air_rapid_aerial"),
            hash40("item_scope_air_fire_aerial"),
            hash40("item_scope_air_end_aerial"),
            hash40("item_scope_start_upper_empty"),
            hash40("item_scope_rapid_empty"),
            hash40("item_scope_rapid_upper_empty"),
            hash40("item_scope_fire_empty"),
            hash40("item_scope_fire_upper_empty"),
            hash40("item_scope_end_empty"),
            hash40("item_scope_end_upper_empty"),
            hash40("item_scope_air_start_empty"),
            hash40("item_scope_air_rapid_empty"),
            hash40("item_scope_air_fire_empty"),
            hash40("item_scope_air_end_empty"),
            hash40("item_scope_air_start_empty_aerial"),
            hash40("item_scope_air_rapid_empty_aerial"),
            hash40("item_scope_air_fire_empty_aerial"),
            hash40("item_scope_air_end_empty_aerial"),
            hash40("item_magicpot_start"),
            hash40("item_magicpot_start_upper"),
            hash40("item_magicpot_wait"),
            hash40("item_magicpot_wait_upper"),
            hash40("item_magicpot_end"),
            hash40("item_magicpot_end_upper"),
            hash40("item_magicpot_fall"),
            hash40("item_magicpot_air_start"),
            hash40("item_magicpot_air_end"),
            hash40("item_magicpot_air_start_aerial"),
            hash40("item_magicpot_air_end_aerial"),
            hash40("item_drill_shoot"),
            hash40("item_drill_shoot_upper"),
            hash40("item_drill_shoot_air"),
            hash40("item_steeldiver_shoot"),
            hash40("item_steeldiver_shoot_upper"),
            hash40("item_steeldiver_shoot_air"),
            hash40("item_staff_shoot"),
            hash40("item_staff_shoot_upper"),
            hash40("item_staff_shoot_air"),
            hash40("item_revengeshooter_shoot"),
            hash40("item_revengeshooter_shoot_upper"),
            hash40("item_revengeshooter_shoot_air"),
            hash40("item_genesis_get"),
            hash40("item_rocketbelt"),
            hash40("item_specialflag_hoist"),
            hash40("item_assist_item"),
            //hash40("item_gekikara_wait"),
            hash40("item_gekikara_wait_face"),
            //hash40("item_grass_pull"),
            hash40("item_genesis_start"),
            hash40("item_genesis_start_upper"),
            hash40("item_genesis_turn"),
            hash40("item_genesis_fire"),
            hash40("item_genesis_loop"),
            hash40("item_genesis_end"),
            hash40("item_genesis_fall"),
            hash40("item_genesis_aerial_legs"),
            hash40("item_genesis_air_fire"),
            hash40("item_genesis_air_loop"),
            hash40("item_genesis_air_end"),
            //hash40("swim_rise"),
            //hash40("swim_up"),
            //hash40("swim_up_damage"),
            //hash40("swim"),
            //hash40("swim_f"),
            //hash40("swim_end"),
            //hash40("swim_turn"),
            //hash40("swim_drown"),
            //hash40("swim_drown_out"),
            //hash40("ladder_wait"),
            //hash40("ladder_up"),
            //hash40("ladder_down"),
            //hash40("ladder_catch_r"),
            //hash40("ladder_catch_l"),
            //hash40("ladder_catch_air_r"),
            //hash40("ladder_catch_air_l"),
            //hash40("ladder_catch_end_r"),
            //hash40("ladder_catch_end_l"),
            hash40("jump_board_jump"),
            hash40("fill_blank"),
            hash40("fill_blank_damage"),
            hash40("fill_blank_finish"),
            hash40("fill_blank_cliff"),
            hash40("fill_blank_sleep"),
        ].contains(&motion){
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodybase"),    false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodyemi"),     false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodydecal"),   false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weaponbase"),  false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weapondecal"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weaponemi"),   false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_headbase"),    false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_heademi"),     false);
            if !WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED){
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_base"),    true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_emi"),     true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_decal"),   true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_base"),  true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_decal"), true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_emi"),   true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("head_base"),    true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("head_emi"),     true);
            }
        }else{
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodybase"),    true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodyemi"),     true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_bodydecal"),   true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weaponbase"),  true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weapondecal"), true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_weaponemi"),   true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_headbase"),    true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("low_heademi"),     true);
            if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED){
                WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_base"),    false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_emi"),     false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("body_decal"),   false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_base"),  false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_decal"), false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("weapon_emi"),   false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("head_base"),    false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("head_emi"),     false);
            }
        }
    }else{
        kamui_on_change_motion_og(vtable,fighter);
    }
}



#[skyline::from_offset(*KAMUI_INIT_OFFSET)]
pub unsafe extern "C" fn kamui_init_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_init(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    if (*fighter).battle_object.kind == *FIGHTER_KIND_KAMUI as u32 {
        let shield_data = ShieldDataResource::new(
            0.0,
            7.0,
            11.0,
            0.0,
            17.0,
            11.0,
            8.0,
            Hash40::new("top"),
            *COLLISION_SHAPE_TYPE_CAPSULE as u8,
            *SHIELD_TYPE_GUARD as u8
        );
        let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource::new(
            shield_datas,
            1,
            1,
            false,
            false,
            true
        );
        
        MiscModule::add_shield_group(module_accessor, resource, 2);
    }
    kamui_init_og(vtable,fighter);
}





#[skyline::from_offset(*KAMUI_ON_SHIELD)]
pub unsafe extern "C" fn kamui_on_shield_original(vtable: u64, fighter: &mut Fighter, collision_event: &mut ShieldAttackCollisionEvent);

pub unsafe extern "C" fn kamui_on_shield(vtable: u64, fighter: &mut Fighter, collision_event: &mut ShieldAttackCollisionEvent ) {

    let module_accessor = fighter.battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let opponent_id =(*collision_event.collision_log).opponent_object_id;
        let opponent_cat =(*collision_event.collision_log).opponent_object_category;
        let opponent_raw_power =collision_event.raw_power;
        let opponent_real_power =collision_event.real_power;
        let opponent_accessor = sv_battle_object::module_accessor(opponent_id);
        let opponent_common = get_fighter_common_from_accessor(&mut* opponent_accessor);



        if opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_WEAPON
        || opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_ITEM{
            if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD
            || StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT{
                if opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_ITEM {
                    let item_manager = singletons::ItemManager();
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,opponent_id);
                }
                let opponent_kind =utility::get_kind( &mut *opponent_accessor);
                WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
                if opponent_kind != *WEAPON_KIND_ROSETTA_TICO{
                    notify_event_msc_cmd!(opponent_common, Hash40::new_raw(0x199c462b5d));
                }else{
                    StatusModule::change_status_request(opponent_accessor, *WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD ,false);
                }
                ShieldModule::clear_all(module_accessor);
                WorkModule::set_float(module_accessor, opponent_real_power, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT);
                ArticleModule::generate_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_RYUSENSYA, false, -1);
                StatusModule::change_status_request(module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT ,false);
            }
        }
    }else{
        kamui_on_shield_original(vtable,fighter,collision_event);
    }
}


#[skyline::from_offset(*KAMUI_ON_DAMAGE)]
pub unsafe extern "C" fn kamui_on_damage_og(vtable: u64, fighter: &mut Fighter,damage_log:u64);

pub unsafe extern "C" fn kamui_on_damage(vtable: u64, fighter: &mut Fighter,damage_log:u64) {
    let module_accessor = fighter.battle_object.module_accessor;

    
    if is_potemkin(module_accessor){
        let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let dmg = *((*(((damage_log as u64) + 0x10) as *const u64) + 0x4) as *const f32);
        let status_kind =StatusModule::status_kind(fighter.battle_object.module_accessor);
        //let collision_log = *((damage_log as *const u64).add(0x28 / 0x8)) as *const CollisionLog;
        let collision_log = *((damage_log as *const u64).add(0x28 / 0x8)) as *const CollisionLog;
        let opponent_id = *(*(damage_log as *mut u64).add(0x10 / 0x8) as *mut u32).add(0x44 / 0x4);
        let category = sv_battle_object::category(opponent_id);
        //let collision_log : &mut CollisionLogScuffed = std::mem::transmute(damage_log);
        println!("part_id = {}",(*collision_log).collider_part_id);
        println!("col_kind = {}",(*collision_log).collision_kind);

        if *(*(damage_log as *const *const u64).offset(0x10 / 0x8) as *const u8).add(0xd0) == 0{
            WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC);
            if [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,
                *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
                FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE,
                FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF,
                FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER,
                ].contains(&status_kind)
            && WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON) {
                let time_base = 45.0 * (1.0/sv_information::slow_rate());
                WorkModule::set_float(module_accessor, time_base, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE);
                WorkModule::set_int(module_accessor, time_base as i32, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
                let agent = get_fighter_common_from_accessor((&mut*module_accessor));
                damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0); 
                
                macros::BURN_COLOR(agent, 24,  1,  1, 0.5);
                WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
            }
        }
        if (*collision_log).collision_kind == *COLLISION_KIND_HIT as u8 {
            //if HitModule::get_status(module_accessor, fighter.battle_object.battle_object_id as i32, 0) == 0{
                UiManager::add_tension(get_ui_index_from_entry_id(entry_id), dmg*0.2);
            //}
        }
    }else{
        kamui_on_damage_og(vtable,fighter,damage_log);
    }
}

#[skyline::from_offset(*KAMUI_ON_ATTACK)]
pub unsafe extern "C" fn kamui_on_attack_og(vtable: u64, fighter: &mut Fighter,collision_log:CollisionLogScuffed, damage: f32);

pub unsafe extern "C" fn kamui_on_attack(vtable: u64, fighter: &mut Fighter,collision_log:CollisionLogScuffed, damage:f32) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let attack_data = AttackModule::attack_data(fighter.battle_object.module_accessor, collision_log.collider_id as i32, false);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(module_accessor);
        let category = sv_battle_object::category(collision_log.opponent_object_id);
        println!("part_id = {}",collision_log.collider_id);
        if category == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && collision_log.collision_kind == *COLLISION_KIND_HIT as u8 {
            let opp_boma = sv_battle_object::module_accessor(collision_log.opponent_object_id as u32);
            let opp_status = StatusModule::status_kind(opp_boma);
            let pot_lr = PostureModule::lr(module_accessor);
            let opp_lr = PostureModule::lr(opp_boma);
    
            if (StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW){
                if HitModule::get_status(opp_boma, collision_log.receiver_id as i32, 0) == 0 {
                    //AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
                    EffectModule::reset_common(opp_boma);
                    EffectModule::req_common(opp_boma,Hash40::new("fire_hit_l_l"),0.0);
                }
            }
    
            if (StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW)
            && ![FIGHTER_STATUS_KIND_POT_REBOUND_JUMP,*FIGHTER_STATUS_KIND_REBOUND,*FIGHTER_STATUS_KIND_CAPTURE_CUT].contains(&opp_status) {
                if HitModule::get_status(opp_boma, collision_log.receiver_id as i32, 0) == 0 {
                    if pot_lr != -opp_lr{
                        PostureModule::set_lr(opp_boma,-pot_lr);
                        PostureModule::update_rot_y_lr(opp_boma);
                        //let lr = PostureModule::lr(enemy);
                        //TurnModule::set_turn(enemy, Hash40::new("back_damage"), -lr, false, false, true);
                    }
                    if collision_log.collider_id == 0{
                        WorkModule::set_float(opp_boma, 0.47, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                    }else{
                        WorkModule::set_float(opp_boma, 0.6, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                    }

                    if StatusModule::situation_kind(opp_boma) != *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(opp_boma, FIGHTER_STATUS_KIND_POT_REBOUND_JUMP, false);
                    }else{
                        StatusModule::change_status_request_from_script(opp_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                    }
                }
            }
            if 
            ![FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL,
            FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK,
            FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP,
            FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START,
            *FIGHTER_KAMUI_STATUS_KIND_FINAL_END,
            FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP,
            FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE,
            *FIGHTER_KAMUI_STATUS_KIND_FINAL_READY,
            *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,
            *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT,
            *FIGHTER_STATUS_KIND_FINAL].contains(&status){
                if HitModule::get_status(opp_boma, collision_log.receiver_id as i32, 0) == 0 {
                    let count = smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager() as _) as f32;
                    let mult = 0.67 - 0.05 * (count-1.0);
                    UiManager::add_tension(get_ui_index_from_entry_id(entry_id), damage*mult);
                }
            }
        }
    }else{
        kamui_on_attack_og(vtable,fighter,collision_log,damage);
    }
}


pub unsafe extern "C" fn kamui_on_status_change(vtable: u64, fighter: &mut Fighter, next_status:i32) {
    let module_accessor = fighter.battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let status = StatusModule::status_kind(module_accessor);
        let force_reset;
        let command_kind;
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_FINAL,
            *FIGHTER_RYU_STATUS_KIND_FINAL2
        ].contains(&status) {
            force_reset = true;
            command_kind = -1;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
            force_reset = false;
            command_kind = 0;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
            force_reset = false;
            command_kind = 1;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND {
            force_reset = false;
            command_kind = 2;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
            force_reset = false;
            command_kind = 3;
        }
        else {
            force_reset = true;
            command_kind = -1;
        }
        let opponent_lr = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
        let lr = PostureModule::lr(module_accessor);
        if opponent_lr != 0.0
        && lr != opponent_lr {
            return;
        }
    
        if command_kind < 0 {
            if !force_reset {
                return;
            }
        }
        else {
            let special_command_lr = ControlModule::get_special_command_lr(module_accessor, command_kind);
            if special_command_lr != 0.0 && special_command_lr != lr {
                PostureModule::set_lr(module_accessor, special_command_lr);
                PostureModule::update_rot_y_lr(module_accessor);
            }
        }
        ControlModule::reset_special_command(module_accessor, false);
    }
}

#[skyline::from_offset(*KAMUI_ON_SITUATION_CHANGE_2)]
pub unsafe extern "C" fn kamui_on_situation_change_og(vtable: *mut u64, fighter: &mut Fighter,something: u64);

pub unsafe extern "C" fn kamui_on_situation_change(vtable: *mut u64, fighter: &mut Fighter, something: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let some_vtable_func: extern "C" fn(*mut u64, &mut Fighter, u64, ...) -> u64 = std::mem::transmute(*((*vtable+0xf8) as *mut u64));
        some_vtable_func(vtable,fighter,something);
        let kinetic_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let situation_kind = *((something+0xc) as *const u32);

        if situation_kind <  4
        && situation_kind != 2
        && *(kinetic_energy as *mut u8).add(0xa4) != 0{
            *(kinetic_energy as *mut u8).add(0xa5) = 0;
        }    
    }else{
        kamui_on_situation_change_og(vtable,fighter,something);
    }
}

#[skyline::from_offset(*KAMUI_ON_STATUS_CHANGE_2)]
pub unsafe extern "C" fn kamui_on_status_change_2_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_status_change_2(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    if is_potemkin(module_accessor){
        let status = StatusModule::status_kind(module_accessor);
        let kinetic_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let is_jump = 
        [
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_WALL_JUMP,
            *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP
        ].contains(&status);

        if status < *FIGHTER_STATUS_KIND_GIMMICK_BARREL{
            if is_jump {
                if *(kinetic_energy as *mut u8).add(0xa4) != 0{
                    *(kinetic_energy as *mut u8).add(0xa5) = 1;
                }
            }
            return;
        }else{
            if status == *FIGHTER_STATUS_KIND_GIMMICK_BARREL{
                let kinetic_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                if *(kinetic_energy as *mut u8).add(0xa4) != 0{
                    *(kinetic_energy as *mut u8).add(0xa5) = 0;
                }
                return;
            }else if status != *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP{
                return;
            }
        }
        if *(kinetic_energy as *mut u8).add(0xa4) != 0{
            *(kinetic_energy as *mut u8).add(0xa5) = 1;
        }
    }else{
        kamui_on_status_change_2_og(vtable,fighter);
    }
    return;
}


#[skyline::from_offset(*KAMUI_ON_DEATH)]
pub unsafe extern "C" fn kamui_on_death_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_death(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    println!("on_death");
    if is_potemkin(module_accessor){
        let kinetic_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if *(kinetic_energy as *mut u8).add(0xa4) != 0{
            *(kinetic_energy as *mut u8).add(0xa5) = 0;
        }
        //*(kinetic_energy as *mut u8).add(0x9c) = 0;
    }else{
        kamui_on_death(vtable,fighter);
    }
}






pub fn install() {
    unsafe {
        fn patch_vtable_function(offset: usize, function: u64) {
            // println!("function ptr: {:#x}", function as u64);
            let low = (function as u64 & 0xFFFFFFFF) as u32;
            // println!("Low to Big Endian: {:#x} > {:#?}", low, low.to_be_bytes());
            let high = (function as u64 >> 32) as u32;
            // println!("High to Big Endian: {:#x} > {:#?}", high, high.to_be_bytes());
            let _ = skyline::patching::Patch::in_text(offset).data(low);
            let _ = skyline::patching::Patch::in_text(offset + 0x4).data(high);
        }

        

        LazyLock::force(&KAMUI_ON_RESET_OFFSET);
        LazyLock::force(&KAMUI_INIT_OFFSET);
        LazyLock::force(&KAMUI_ON_SHIELD);
        LazyLock::force(&KAMUI_ON_DAMAGE);
        LazyLock::force(&KAMUI_ON_ATTACK);
        LazyLock::force(&KAMUI_ON_CHANGE_MOTION);
        LazyLock::force(&KAMUI_ON_SITUATION_CHANGE_2);
        LazyLock::force(&KAMUI_ON_STATUS_CHANGE_2);
        LazyLock::force(&KAMUI_ON_DEATH);

        patch_vtable_function(*OFFSET_ON_FRAME_4                , kamui_on_frame as u64);
        patch_vtable_function(*OFFSET_ON_SEARCH                 , kamui_on_search as u64);
        patch_vtable_function(*OFFSET_RESET_1                   , kamui_reset as u64);
        patch_vtable_function(*OFFSET_INITIALIZE                , kamui_init as u64);
        patch_vtable_function(*OFFSET_ON_SHIELD_COLLISION       , kamui_on_shield as u64);
        patch_vtable_function(*OFFSET_ON_DAMAGE                 , kamui_on_damage as u64);
        patch_vtable_function(*OFFSET_ON_ATTACK                 , kamui_on_attack as u64);
        patch_vtable_function(*OFFSET_ON_CHANGE_MOTION          , kamui_on_change_motion as u64);
        patch_vtable_function(*OFFSET_ON_STATUS_CHANGE          , kamui_on_status_change as u64);
        patch_vtable_function(*OFFSET_ON_SITUATION_CHANGE_2     , kamui_on_situation_change as u64);
        patch_vtable_function(*OFFSET_ON_STATUS_CHANGE_2        , kamui_on_status_change_2 as u64);
        patch_vtable_function(*OFFSET_ON_DEATH                  , kamui_on_death as u64);

    }
}